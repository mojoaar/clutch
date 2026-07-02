<script lang="ts">
  import type { Message } from '$lib/stores/chat';
  import { chatStore } from '$lib/stores/chat';
  import { onMount } from 'svelte';
  import { devMode } from '$lib/stores/dev';
  import * as db from '$lib/db';
  import MarkdownRenderer from './MarkdownRenderer.svelte';
  import Avatar from './Avatar.svelte';
  import AIAvatar from './AIAvatar.svelte';
  import Timestamp from './Timestamp.svelte';
  import { Copy, Check, RefreshCw } from '@lucide/svelte';
  import { PROVIDER_COLORS } from '$lib/utils/provider-colors';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    message: Message;
    isStreaming?: boolean;
    onRegenerate?: (messageId: string) => void;
  }

  let { message, isStreaming = false, onRegenerate }: Props = $props();

  let isHovered = $state(false);
  let isCopied = $state(false);

  let profileName = $state('');
  let profileColor = $state('var(--primary)');
  let profileImageUrl = $state('');

  onMount(async () => {
    try {
      const profile = await db.getUserProfile();
      if (profile) {
        profileName = profile.displayName;
        profileImageUrl = profile.avatarType === 'image' ? profile.avatarData : '';
        profileColor = profile.avatarColor;
      }
    } catch (e) { console.debug('Failed to load user profile', e); }
  });

  function copyMessage() {
    navigator.clipboard.writeText(message.content).then(() => {
      isCopied = true;
      setTimeout(() => (isCopied = false), 2000);
    });
  }

  let avatarColor = $derived(
    message.role === 'user'
      ? 'var(--primary)'
      : (message.provider ? (PROVIDER_COLORS[message.provider] ?? 'var(--primary)') : 'var(--primary)'),
  );
</script>

{#if !message.isDeleted}
  <div
    class="message message--{message.role}"
    class:message--streaming={$chatStore.streamingStatus === 'streaming'}
    role="article"
    aria-label={message.role === 'user' ? $LL.chat.you() + ' message' : $LL.chat.assistant() + ' message'}
    onmouseenter={() => (isHovered = true)}
    onmouseleave={() => (isHovered = false)}
  >
    <div class="message__avatar">
      {#if message.role === 'user'}
        <Avatar
          size={32}
          name={profileName || undefined}
          imageUrl={profileImageUrl || undefined}
          color={profileColor}
        />
      {:else}
        <AIAvatar provider={message.provider} size={32} />
      {/if}
    </div>

    <div class="message__body">
      <div class="message__header">
        <span class="message__role">
          {message.role === 'user' ? (profileName ? `${$LL.chat.you()} (${profileName})` : $LL.chat.you()) : $LL.chat.assistant()}
          {#if message.model}
            <span class="message__model">· {message.model}</span>
          {/if}
        </span>
        <Timestamp date={message.createdAt} />
      </div>

      <div class="message__content">
        {#if message.role === 'assistant'}
          {#if isStreaming}
            <p class="message__text">{message.content}</p>
          {:else}
            <MarkdownRenderer content={message.content} />
          {/if}
        {:else}
          <p class="message__text">{message.content}</p>
        {/if}

        {#if message.editedAt}
          <span class="message__edited-label">{$LL.chat.edited()}</span>
        {/if}
      </div>

      {#if $devMode && message.tokensUsed !== undefined}
        <span class="message__tokens">{message.tokensUsed} {$LL.streamingTokens()}</span>
      {/if}
    </div>

    <div class="message__actions" class:message__actions--visible={isHovered}>
      <button class="message__action" onclick={copyMessage} aria-label={$LL.aria.copyMessage()}>
        {#if isCopied}
          <Check size={15} strokeWidth={1.5} />
        {:else}
          <Copy size={15} strokeWidth={1.5} />
        {/if}
      </button>
      {#if message.role === 'assistant' && !isStreaming && onRegenerate}
        <button class="message__action" onclick={() => onRegenerate(message.id)} aria-label={$LL.aria.regenerateResponse()}>
          <RefreshCw size={15} strokeWidth={1.5} />
        </button>
      {/if}
    </div>
  </div>
{:else}
  <div class="message message--deleted" role="article">
    <div class="message__avatar">
      {#if message.role === 'user'}
        <Avatar size={32} />
      {:else}
        <AIAvatar size={32} />
      {/if}
    </div>
    <div class="message__body">
      <span class="message__deleted-text">{$LL.chat.deleted()}</span>
    </div>
  </div>
{/if}

<style>
  .message {
    display: flex;
    gap: 14px;
    padding: 16px 20px;
    transition: background 0.15s;
  }

  .message:hover {
    background: var(--surface);
  }

  .message--deleted {
    opacity: 0.5;
  }

  .message--user {
    flex-direction: row-reverse;
    text-align: right;
  }

  .message--user .message__body {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .message--user .message__header {
    flex-direction: row-reverse;
    justify-content: flex-end;
  }

  .message--assistant {
    flex-direction: row;
  }

  .message__avatar {
    flex-shrink: 0;
    padding-top: 2px;
  }

  .message__body {
    flex: 1;
    min-width: 0;
  }

  .message__header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 6px;
  }

  .message__role {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .message__model {
    font-weight: 400;
    color: var(--text-dim);
  }

  .message__content {
    color: var(--text);
  }

  .message__text {
    margin: 0;
    font-size: 14px;
    line-height: 1.7;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .message__edited-label {
    display: inline-block;
    margin-top: 4px;
    font-size: 11px;
    color: var(--text-dim);
    font-style: italic;
  }

  .message__tokens {
    display: inline-block;
    margin-top: 6px;
    font-size: 11px;
    color: var(--text-dim);
  }

  .message__deleted-text {
    font-size: 13px;
    font-style: italic;
    color: var(--text-dim);
  }

  .message__actions {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex-shrink: 0;
    padding-top: 2px;
    width: 28px;
    opacity: 0;
    transition: opacity 0.15s;
  }

  .message__actions--visible {
    opacity: 1;
  }

  .message__action {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .message__action:hover {
    background: var(--surface-hover);
    color: var(--text);
  }
</style>
