import { invoke } from '@tauri-apps/api/core';
import { Channel } from '@tauri-apps/api/core';
import type { Message } from '$lib/stores/chat';
import { chatStore } from '$lib/stores/chat';
import { v4 as uuid } from 'uuid';
import { get } from 'svelte/store';
import * as db from '$lib/db';
import { network, registerQueueProcessor } from '$lib/services/network';
import { addToast } from '$lib/stores/toast';
import LL from '$lib/i18n/i18n-svelte';

export interface StreamChatParams {
  provider: string;
  model: string;
  messages: Array<{ role: string; content: string }>;
  sessionId: string;
  systemPrompt?: string;
  temperature?: number;
  maxTokens?: number;
}

export async function streamChat(params: StreamChatParams): Promise<void> {
  const assistantId = uuid();
  const assistantMessage: Message = {
    id: assistantId,
    sessionId: params.sessionId,
    role: 'assistant',
    content: '',
    createdAt: new Date().toISOString(),
    provider: params.provider,
    model: params.model,
  };

  chatStore.addMessage(assistantMessage);
  chatStore.setStreamingStatus('streaming');
  chatStore.setStreamingTokens(0);

  const channel = new Channel<string>();
  let streamedContent = '';
  let wasInterrupted = false;

  channel.onmessage = (data: string) => {
    if (data.startsWith('__TOKENS__:')) {
      const tokens = parseInt(data.split(':')[1], 10);
      if (!isNaN(tokens)) {
        chatStore.setStreamingTokens(tokens);
      }
      if (!wasInterrupted) chatStore.setStreamingStatus('complete');
    } else if (data === '__STREAM_INTERRUPTED__') {
      wasInterrupted = true;
      chatStore.setStreamingStatus('interrupted');
    } else {
      streamedContent += data;
      chatStore.appendToMessage(assistantId, data);
    }
  };

  try {
    await invoke('stream_chat', {
      channel,
      request: {
        provider: params.provider,
        model: params.model,
        messages: params.messages,
        systemPrompt: params.systemPrompt ?? null,
        temperature: params.temperature ?? null,
        maxTokens: params.maxTokens ?? null,
      },
    });
    db.createMessage(assistantId, params.sessionId, 'assistant', streamedContent, 0);
    if (!wasInterrupted) chatStore.setStreamingStatus('complete');
  } catch (error) {
    const errMsg = typeof error === 'string' ? error : error instanceof Error ? error.message : 'API error';

    const isRetryable =
      errMsg.toLowerCase().includes('network') ||
      errMsg.toLowerCase().includes('timeout') ||
      errMsg.toLowerCase().includes('connection') ||
      errMsg.toLowerCase().includes('abort') ||
      errMsg.toLowerCase().includes('fetch');

    if (isRetryable) {
      const { get } = await import('svelte/store');
      if (get(network.queueSize) < 50) {
        network.enqueue('retry_stream', [params], 3);
        chatStore.appendToMessage(assistantId, '\n\n' + 'Message queued for retry');
        chatStore.setStreamingStatus('complete');
        return;
      }
    }

    chatStore.appendToMessage(assistantId, '\n\n' + errMsg);
    db.createMessage(assistantId, params.sessionId, 'assistant', errMsg, 0);
    chatStore.setStreamingStatus('complete');
    console.debug('streamChat failed:', error);
  }
}

export async function sendMessage(
  sessionId: string,
  content: string,
  provider: string,
  model: string,
  previousMessages: Message[],
  systemPrompt?: string,
  contextContent?: string,
): Promise<void> {
  if (typeof navigator !== 'undefined' && !navigator.onLine) {
    network.enqueue('sendMessage', [sessionId, content, provider, model, previousMessages, systemPrompt, contextContent], 3);
    addToast(get(LL).networkStatus.messageQueued(), 'info', 5000);
    return;
  }

  const userMessage: Message = {
    id: uuid(),
    sessionId,
    role: 'user',
    content,
    createdAt: new Date().toISOString(),
  };

  chatStore.addMessage(userMessage);
  db.createMessage(userMessage.id, sessionId, 'user', content, 0);

  const apiMessages = [...previousMessages, userMessage].map((m) => ({
    role: m.role,
    content: m.content,
  }));

  if (contextContent) {
    apiMessages.push({ role: 'system', content: `The following web content was fetched for context:\n${contextContent}` });
  }

  await streamChat({
    provider,
    model,
    messages: apiMessages,
    sessionId,
    systemPrompt,
  });
}

registerQueueProcessor(async (item) => {
  if (item.operation === 'sendMessage') {
    const [sessionId, content, provider, model, previousMessages, systemPrompt, contextContent] = item.args;
    const apiMessages = previousMessages.map((m: Message) => ({ role: m.role, content: m.content }));
    if (contextContent) {
      apiMessages.push({ role: 'system', content: `The following web content was fetched for context:\n${contextContent}` });
    }
    await streamChat({ provider, model, messages: apiMessages, sessionId, systemPrompt });
  } else if (item.operation === 'retry_stream') {
    const [params] = item.args as [StreamChatParams];
    await streamChat(params);
  }
});
