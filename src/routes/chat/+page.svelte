<script lang="ts">
  import SessionSidebar from '$lib/components/SessionSidebar.svelte';
  import VirtualChatList from '$lib/components/VirtualChatList.svelte';
  import ChatInput from '$lib/components/ChatInput.svelte';
  import ContextBar from '$lib/components/ContextBar.svelte';
  import NetworkStatus from '$lib/components/NetworkStatus.svelte';
  import { chatStore, type Message } from '$lib/stores/chat';
  import { sendMessage } from '$lib/services/api';
  import { fetchUrl } from '$lib/services/web-fetcher';
  import { v4 as uuid } from 'uuid';
  import { readTextFile, readDir } from '@tauri-apps/plugin-fs';
  import { homeDir } from '@tauri-apps/api/path';
import { PROVIDERS } from '$lib/services/providers';
import { getCachedModels, type ModelInfo } from '$lib/services/models';
import { getSetting, setSetting } from '$lib/services/settings';
import { createNewSession } from '$lib/services/sessions';
  import { invoke } from '@tauri-apps/api/core';
  import { goto, afterNavigate } from '$app/navigation';
  import {
    Archive,
    Trash2,
    X,
    ListChecks,
  } from '@lucide/svelte';
  import * as db from '$lib/db';
  import LL from '$lib/i18n/i18n-svelte';


  $effect(() => {
    db.listSessions(true).then((sessions) => {
      chatStore.setSessions(sessions);
    });
  });

  $effect(() => {
    const sid = $chatStore.activeSessionId;
    if (sid && sid !== loadedMessagesFor) {
      loadedMessagesFor = sid;
      db.listMessages(sid).then((messages) => {
        chatStore.setMessages(messages);
      });
    }
  });

  let currentModel = $derived(
    $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId)?.model ?? 'deepseek-v4-pro',
  );

  let currentProvider = $derived(
    $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId)?.provider ?? 'deepseek',
  );

  const visibleMessages = $derived(
    $chatStore.messages.filter((m) => !m.isDeleted),
  );

  const streamingMessageId = $derived(
    $chatStore.streamingStatus === 'streaming'
      ? visibleMessages.filter((m) => m.role === 'assistant').at(-1)?.id ?? null
      : null,
  );

  const contextTokens = $derived(
    $chatStore.messages
      .filter((m) => m.sessionId === $chatStore.activeSessionId && !m.isDeleted)
      .reduce((sum, m) => sum + (m.tokensUsed || 0), 0) +
      ($chatStore.streamingStatus === 'streaming' ? $chatStore.streamingTokens : 0),
  );


  let availableModels = $state<string[]>([]);
  let configured = $state(false);
  let loadedMessagesFor = $state('');
  let contextLimit = $state(8192);
  let sidebarWidth = $state(260);
  let isResizing = $state(false);

  $effect(() => {
    const sid = $chatStore.activeSessionId;
    if (sid) {
      const session = $chatStore.sessions.find((s) => s.id === sid);
      if (session) {
        getCachedModels(session.provider)
          .then((models) => {
            availableModels = models.map((m: ModelInfo) => m.id);
            if (models.length > 0 && !models.find((m: ModelInfo) => m.id === session.model)) {
              availableModels.push(session.model);
            }
          })
          .catch(() => {
            availableModels = [session.model];
          });
      }
    }
  });

  async function checkApiKey(providerId?: string) {
    if (providerId) {
      const key = await getSetting(`api_key_${providerId}`);
      configured = !!(key && key.length > 0);
    } else {
      for (const p of Object.values(PROVIDERS)) {
        const key = await getSetting(`api_key_${p.id}`);
        if (key && key.length > 0) {
          configured = true;
          return;
        }
      }
      configured = false;
    }
  }

  $effect(() => {
    const session = $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId);
    checkApiKey(session?.provider);
  });

  afterNavigate(async () => {
    const session = $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId);
    await checkApiKey(session?.provider);
  });

  $effect(() => {
    function handleKeydown(e: KeyboardEvent) {
      if (e.metaKey && e.key === 'n') {
        e.preventDefault();
        handleNewChat();
      }
      if (e.metaKey && e.shiftKey && e.key === 'a') {
        e.preventDefault();
        const id = $chatStore.activeSessionId;
        if (id) {
          chatStore.archiveSession(id);
          db.archiveSession(id);
        }
      }
      if (e.metaKey && e.key === 'Backspace') {
        e.preventDefault();
        const id = $chatStore.activeSessionId;
        if (id && window.confirm($LL.deleteConfirmChat())) {
          chatStore.removeSession(id);
          db.deleteSession(id);
        }
      }
      if (e.metaKey && e.key === 'e') {
        e.preventDefault();
        const id = $chatStore.activeSessionId;
        if (id) {
          const session = $chatStore.sessions.find((s) => s.id === id);
          const title = window.prompt($LL.renameChatPrompt(), session?.title || '');
          if (title && title.trim()) {
            chatStore.updateSession(id, { title: title.trim() });
            db.updateSessionTitle(id, title.trim());
          }
        }
      }
      if (e.metaKey && e.shiftKey && e.key === 'p') {
        e.preventDefault();
        const id = $chatStore.activeSessionId;
        if (id) {
          const session = $chatStore.sessions.find((s) => s.id === id);
          if (session?.isPinned) {
            chatStore.unpinSession(id);
            db.unpinSession(id);
          } else {
            chatStore.pinSession(id);
            db.pinSession(id);
          }
        }
      }
      if (e.metaKey && e.shiftKey && e.key === 'c') {
        e.preventDefault();
        const messages = $chatStore.messages;
        if (messages.length > 0) {
          const text = messages
            .map((m) => {
              const label = m.role === 'user' ? $LL.chat.you() : m.role === 'assistant' ? $LL.chat.assistant() : 'System';
              return `${label}: ${m.content}`;
            })
            .join('\n\n');
          navigator.clipboard.writeText(text);
        }
      }
    }
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });

  $effect(() => {
    invoke<number>('get_context_limit', { model: currentModel })
      .then((n) => { contextLimit = n; })
      .catch((e) => { console.debug('Failed to get context limit', e); });
  });

  let lastInterruptedId = $derived.by(() => {
    const msgs = $chatStore.messages;
    for (let i = msgs.length - 1; i >= 0; i--) {
      if (msgs[i].role === 'assistant' && msgs[i].content.length > 0 && !msgs[i].isDeleted) {
        return msgs[i].id;
      }
    }
    return '';
  });

  function isTextExtension(name: string): boolean {
    return /\.(txt|md|csv|log|json|xml|ya?ml|tsx?|jsx?|rs|py|go|rb|java|c|cpp|h|hpp|css|s[ac]ss|html?|svg|sh|bash|toml|ini|cfg|env|sql|svelte|vue)$/i.test(name);
  }

  async function handleSend(content: string, files: File[]) {
    let contextContent = '';

    try {
      if (files.length > 0) {
        const names = files.map((f) => f.name).join(', ');
        content = content ? `${content}\n\n📎 ${names}` : `📎 ${names}`;

        const fileContents = await Promise.all(
          files.map(async (f) => {
            try {
              if (f.type && !f.type.startsWith('text/') && !isTextExtension(f.name)) {
                return `\n\n--- File: ${f.name} (binary, not displayed) ---`;
              }
              const text = await f.text();
              return `\n\n--- File: ${f.name} ---\n${text}`;
            } catch (e) { console.debug('Failed to read attached file text', e); return ''; }
          }),
        );
        contextContent += fileContents.filter(Boolean).join('');
      }

      const urlRegex = /https?:\/\/[^\s]+/g;
    const urls = content.match(urlRegex);

    if (urls) {
      const results = await Promise.allSettled(
        urls.map((url) => fetchUrl(url, { mode: 'markdown', maxSizeBytes: 500000 }).catch(() => null)),
      );
      const fetched = results
        .filter((r) => r.status === 'fulfilled' && r.value)
        .map((r: any) => `\n\n--- Fetched from ${r.value.url} ---\n${r.value.content}`);
      if (fetched.length > 0) {
        contextContent += fetched.join('');
      }
    }

    const pathRegex = /(~?\/[\w.\/\-]+|\.\/[\w.\/\-]+|\.\.\/[\w.\/\-]+)/g;
    const filePaths = content.match(pathRegex);

    if (filePaths) {
      const hd = await homeDir();
      for (const rawPath of filePaths) {
        const path = rawPath.replace('~', hd);
        const blockedPatterns = ['/.ssh/', '/.gnupg/', '/.aws/', '/Library/Keychains/', '/.local/share/keyrings/', '/.config/gh/'];
        if (blockedPatterns.some((p) => path.includes(p))) continue;
        try {
          const fileContent = await readTextFile(path);
          contextContent += `\n\n--- Content of ${rawPath} ---\n${fileContent}`;
        } catch (e) {
          console.debug('Failed to read file at path', rawPath, e);
          try {
            const entries = await readDir(path);
            const listing = entries.map((f: any) => `${f.isDirectory ? '📁' : '📄'} ${f.name}`).join('\n');
            contextContent += `\n\n--- Contents of ${rawPath} ---\n${listing}`;
          } catch (e2) { console.debug('Failed to read directory at path', rawPath, e2); }
        }
      }
    }

    const session = $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId);

      const model = session?.model ?? 'deepseek-chat';
      const prevMessages = $chatStore.messages.filter((m) => !m.isDeleted);

      let trimmedMessages: Array<{ role: string; content: string }> = prevMessages.map((m) => ({
        role: m.role,
        content: m.content,
      }));

      try {
        const limit = await invoke<number>('get_context_limit', { model });
        const msgTuples: [string, string][] = prevMessages.map(m => [m.role, m.content] as [string, string]);
        const trimmed = await invoke<[string, string][]>('auto_trim_context', {
          messages: msgTuples,
          systemPrompt: session?.systemPrompt ?? null,
          contextContent: contextContent || null,
          model,
          keepLast: 4,
        });
        trimmedMessages = trimmed.map(([role, content]) => ({ role, content }));
      } catch { /* trim failed — use untrimmed messages */ }

      await sendMessage(
        $chatStore.activeSessionId ?? '',
        content,
        session?.provider ?? 'deepseek',
        model,
        trimmedMessages as Message[],
        session?.systemPrompt,
        contextContent || undefined,
      );
    } catch (e) {
      const msg = typeof e === 'string' ? e : e instanceof Error ? e.message : 'Send failed';
      chatStore.addMessage({
        id: uuid(),
        sessionId: $chatStore.activeSessionId ?? '',
        role: 'system',
        content: $LL.somethingWentWrong() + ': ' + msg,
        createdAt: new Date().toISOString(),
      });
    }
  }

  async function handleStop() {
    try { await invoke('abort_stream'); } catch { /* ignore — command may not be registered */ }
    chatStore.setStreamingStatus('idle');
  }

  async function handleRegenerate(assistantMessageId: string) {
    const msgs = $chatStore.messages;
    const idx = msgs.findIndex(m => m.id === assistantMessageId);
    const prevUserMsg = [...msgs.slice(0, idx)].reverse().find(m => m.role === 'user');
    if (!prevUserMsg) return;
    chatStore.markMessageDeleted(assistantMessageId);
    db.deleteMessage(assistantMessageId);
    const session = $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId);
    await sendMessage(
      $chatStore.activeSessionId ?? '',
      prevUserMsg.content,
      session?.provider ?? 'deepseek',
      session?.model ?? 'deepseek-chat',
      $chatStore.messages.filter((m) => !m.isDeleted && m.id !== assistantMessageId),
      session?.systemPrompt,
    );
  }

  async function handleBulkArchive() {
    const ids = [...$chatStore.selectedSessionIds];
    for (const id of ids) {
      await db.archiveSession(id);
    }
    chatStore.bulkArchive();
  }

  async function handleBulkDelete() {
    const ids = [...$chatStore.selectedSessionIds];
    for (const id of ids) {
      await db.deleteSession(id);
    }
    chatStore.bulkDelete();
  }

  async function handleProviderChange(providerId: string) {
    const session = $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId);
    if (!session) return;
    const p = PROVIDERS[providerId];
    if (!p) return;
    const savedModel = await getSetting(`default_model_${p.id}`);
    const model = savedModel || p.defaultModel;
    chatStore.updateSession(session.id, { provider: p.id, model });
    await db.updateSessionProvider(session.id, p.id, model);
  }

  async function handleModelChange(modelId: string) {
    const session = $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId);
    if (!session) return;
    chatStore.updateSession(session.id, { model: modelId });
    await db.updateSessionProvider(session.id, session.provider, modelId);
  }

  $effect(() => {
    getSetting('sidebar_width').then((val) => {
      const n = val ? parseInt(val, 10) : 260;
      if (n >= 180 && n <= 500) sidebarWidth = n;
    }).catch((e) => { console.debug('Failed to load sidebar_width setting', e); });
  });

  function handleResizeStart(e: MouseEvent) {
    isResizing = true;
    const startX = e.clientX;
    const startWidth = sidebarWidth;
    const onMove = (e: MouseEvent) => {
      sidebarWidth = Math.max(180, Math.min(500, startWidth + e.clientX - startX));
    };
    const onUp = () => {
      isResizing = false;
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
      setSetting('sidebar_width', String(sidebarWidth)).catch((e) => { console.debug('Failed to save sidebar_width setting', e); });
    };
    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
  }

  async function handleNewChat() {
    await createNewSession($LL.chat.newChat());
  }
</script>

<div class="chat-layout" class:chat-layout--resizing={isResizing}>
  <div class="sidebar-wrapper" style="width: {sidebarWidth}px">
    <SessionSidebar />
  </div>
  <div class="resize-handle" class:resize-handle--active={isResizing} role="separator" tabindex="0" aria-label="Resize sidebar" onmousedown={handleResizeStart} onkeydown={(e) => e.key === 'Enter' && handleResizeStart(e as unknown as MouseEvent)}></div>
  <div class="chat-main">
    {#if $chatStore.activeSessionId}
      <div class="chat__header">
        <div class="chat__header-info">
          {#if $chatStore.selectMode}
            <span class="chat__header-select-count">
              {$chatStore.selectedSessionIds.size} selected
            </span>
          {:else}
            {@const session = $chatStore.sessions.find((s) => s.id === $chatStore.activeSessionId)}
            {#if session}
              <span class="chat__header-title">{session.title}</span>
              <div class="chat__header-selectors">
                <select
                  class="chat__header-select"
                  value={session.provider}
                  onchange={(e) => handleProviderChange(e.currentTarget.value)}
                >
                  {#each Object.values(PROVIDERS).filter((p) => p.enabled || p.id === session.provider) as p}
                    <option value={p.id}>{p.name}</option>
                  {/each}
                </select>
                <select
                  class="chat__header-select"
                  value={session.model}
                  onchange={(e) => handleModelChange(e.currentTarget.value)}
                >
                  {#each availableModels.length > 0 ? availableModels : [session.model] as m}
                    <option value={m}>{m}</option>
                  {/each}
                </select>
              </div>
            {/if}
          {/if}
        </div>

        <div class="chat__header-actions">
          {#if $chatStore.selectMode}
            <button class="chat__header-btn" onclick={handleBulkArchive} aria-label={$LL.aria.archiveSelected()}>
              <Archive size={15} strokeWidth={1.5} />
              <span>{$LL.archive()}</span>
            </button>
            <button class="chat__header-btn chat__header-btn--danger" onclick={handleBulkDelete} aria-label={$LL.aria.deleteSelected()}>
              <Trash2 size={15} strokeWidth={1.5} />
              <span>{$LL.delete()}</span>
            </button>
            <button class="chat__header-btn" onclick={() => chatStore.toggleSelectMode()} aria-label={$LL.aria.cancelSelection()}>
              <X size={15} strokeWidth={1.5} />
              <span>{$LL.cancel()}</span>
            </button>
          {:else}
            <button
              class="chat__header-btn"
              onclick={() => chatStore.toggleSelectMode()}
              aria-label={$LL.aria.selectConversations()}
            >
              <ListChecks size={15} strokeWidth={1.5} />
            </button>
          {/if}
        </div>
      </div>

      <VirtualChatList
        messages={visibleMessages}
        streamingMessageId={streamingMessageId}
        lastInterruptedId={lastInterruptedId}
        onRegenerate={handleRegenerate}
      />

      <div class="chat__input-area">
        <ContextBar model={currentModel} maxTokens={contextLimit} currentTokens={contextTokens} />
        <ChatInput
          sessionId={$chatStore.activeSessionId}
          provider={currentProvider}
          model={currentModel}
          disabled={$chatStore.streamingStatus === 'streaming'}
          onSend={handleSend}
          onStop={handleStop}
        />
      </div>
    {:else}
      <div class="chat__empty">
        <div class="welcome">
          <div class="welcome__icon">
            <img src="/icon.png" alt="" />
          </div>
          <h1 class="welcome__title">{$LL.welcome.title()}</h1>
          <p class="welcome__tagline">{$LL.welcome.tagline()}</p>
          <p class="welcome__hint">{$LL.welcome.hint()}</p>
          <button class="welcome__cta" onclick={handleNewChat}>
            {$LL.chat.newChat()}
          </button>
          <button class="welcome__settings" onclick={() => goto('/settings?tab=providers')}>
            {$LL.welcome.configureProviders()}
          </button>
        </div>
      </div>
    {/if}

    <NetworkStatus provider={currentProvider} model={currentModel} {configured} />
  </div>
</div>

<style>
  .chat-layout {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .chat-main {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    position: relative;
  }

  .chat__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    border-bottom: 1px solid var(--border);
    background: var(--bg);
    flex-shrink: 0;
  }

  .chat__header-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }

  .chat__header-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .chat__header-selectors {
    display: flex;
    gap: 6px;
    margin-top: 2px;
  }

  .chat__header-select {
    padding: 2px 6px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg);
    color: var(--text-dim);
    font-family: inherit;
    font-size: 11px;
    outline: none;
    min-width: 0;
  }

  .chat__header-select:focus {
    border-color: var(--primary);
  }

  .chat__header-select-count {
    font-size: 13px;
    color: var(--primary);
    font-weight: 600;
  }

  .chat__header-actions {
    display: flex;
    gap: 6px;
    flex-shrink: 0;
  }

  .chat__header-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
  }

  .chat__header-btn:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .chat__header-btn--danger:hover {
    background: color-mix(in srgb, var(--error) 10%, transparent);
    color: var(--error);
  }

  .chat__messages {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
  }

  .chat__input-area {
    flex-shrink: 0;
  }

  .chat__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
  }

  .welcome {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 40px 24px;
  }

  .welcome__icon {
    width: 72px;
    height: 72px;
    margin: 0 auto 20px;
  }

  .welcome__icon img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .welcome__title {
    margin: 0 0 4px;
    font-size: 28px;
    font-weight: 700;
    color: var(--text);
    letter-spacing: -0.02em;
  }

  .welcome__tagline {
    margin: 0 0 28px;
    font-size: 15px;
    color: var(--text-muted);
  }

  .welcome__hint {
    margin: 0 0 16px;
    font-size: 13px;
    color: var(--text-dim);
  }

  .welcome__cta {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 24px;
    border: none;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    background: var(--primary);
    color: white;
    font-family: inherit;
  }

  .welcome__cta:hover {
    background: var(--primary-hover);
  }

  .welcome__settings {
    margin-top: 12px;
    font-size: 12px;
    color: var(--text-dim);
    border: none;
    background: none;
    cursor: pointer;
    font-family: inherit;
  }

  .welcome__settings:hover {
    color: var(--primary);
  }

  .chat-layout--resizing {
    user-select: none;
    cursor: col-resize;
  }

  .sidebar-wrapper {
    flex-shrink: 0;
    overflow: hidden;
  }

  .resize-handle {
    width: 4px;
    flex-shrink: 0;
    background: transparent;
    cursor: col-resize;
    transition: background 0.15s;
  }

  .resize-handle:hover,
  .resize-handle--active {
    background: var(--primary);
  }
</style>
