<script lang="ts">
  import LL from '$lib/i18n/i18n-svelte';
  import { chatStore } from '$lib/stores/chat';
  import type { Session } from '$lib/stores/chat';
  import { goto } from '$app/navigation';
  import { devMode } from '$lib/stores/dev';
  import { addToast } from '$lib/stores/toast';
import * as db from '$lib/db';
import { getSetting } from '$lib/services/settings';
import { createNewSession } from '$lib/services/sessions';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import {
    MessageSquare,
    Pin,
    PinOff,
    Archive,
    Trash2,
    CheckSquare,
    Square,
    MoreHorizontal,
    FileText,
  } from '@lucide/svelte';

  function autofocus(el: HTMLInputElement) {
    el.focus();
  }

  let showArchived = $state(false);

  let contextMenuSessionId = $state<string | null>(null);
  let contextMenuPosition = $state({ x: 0, y: 0 });
  let editingSessionId = $state<string | null>(null);
  let editTitle = $state('');

  const sessions = $derived(
    $chatStore.sessions
      .filter((s) => (showArchived ? true : !s.isArchived))
      .sort((a, b) => {
        if (a.isPinned && !b.isPinned) return -1;
        if (!a.isPinned && b.isPinned) return 1;
        if (!a.isArchived && b.isArchived) return -1;
        if (a.isArchived && !b.isArchived) return 1;
        return new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime();
      }),
  );

  function selectSession(sessionId: string) {
    if ($chatStore.selectMode) {
      chatStore.toggleSessionSelected(sessionId);
    } else {
      chatStore.setActiveSession(sessionId);
    }
  }

  function startRename(session: Session) {
    editingSessionId = session.id;
    editTitle = session.title;
  }

  async function saveRename() {
    if (editingSessionId && editTitle.trim()) {
      chatStore.updateSession(editingSessionId, { title: editTitle.trim() });
      await db.updateSessionTitle(editingSessionId, editTitle.trim());
    }
    editingSessionId = null;
  }

  function cancelRename() {
    editingSessionId = null;
  }

  async function handlePin(session: Session) {
    if (session.isPinned) {
      chatStore.unpinSession(session.id);
      await db.unpinSession(session.id);
    } else {
      chatStore.pinSession(session.id);
      await db.pinSession(session.id);
    }
  }

  async function handleArchive(session: Session) {
    if (session.isArchived) {
      chatStore.unarchiveSession(session.id);
      await db.unarchiveSession(session.id);
    } else {
      chatStore.archiveSession(session.id);
      await db.archiveSession(session.id);
    }
  }

  
  async function handleCopyConversation(session: Session) {
    try {
      let msgs = await db.listMessages(session.id);
      if (msgs.length === 0) {
        msgs = $chatStore.messages
          .filter((m) => m.sessionId === session.id)
          .map((m) => ({ role: m.role, content: m.content, isDeleted: m.isDeleted }));
      }
      const text = (msgs as any[])
        .filter((m: any) => !m.isDeleted)
        .map((m: any) => {
          const role = m.role === "user" ? $LL.chat.you() : $LL.chat.assistant();
          return role + "\n" + m.content;
        })
        .join("\n\n");
      await navigator.clipboard.writeText(text);
      addToast($LL.toast.copySuccess(), "info");
    } catch {
      addToast($LL.toast.copyFailed(), "error");
    }
  }

  async function handleExport(session: Session) {
    try {
      const format = (await getSetting('export_format')) || 'markdown';
      const metadata = (await getSetting('export_metadata')) !== 'false';
      const timestamps = (await getSetting('export_timestamps')) !== 'false';
      const providerInfo = (await getSetting('export_provider_info')) !== 'false';

      const content = await invoke<string>('export_session', {
        sessionId: session.id,
        options: { format, includeMetadata: metadata, includeTimestamps: timestamps, includeProviderInfo: providerInfo, includeSystemPrompt: false },
      });

      const ext = { markdown: 'md', text: 'txt', json: 'json', html: 'html' }[format] || 'md';
      const path = await save({ defaultPath: `${session.title}.${ext}`, filters: [{ name: format, extensions: [ext] }] });
      if (path) {
        await writeTextFile(path, content);
        addToast($LL.toast.exportSuccess(), 'info');
      }
    } catch (e) {
      console.error('Export failed:', e);
      addToast($LL.toast.exportFailed(), 'error');
    }
  }

  async function handleDelete(session: Session) {
    chatStore.removeSession(session.id);
    await db.deleteSession(session.id);
  }

  async function newChat() {
    await createNewSession($LL.chat.newChat());
    goto('/chat');
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr);
    const now = new Date();
    const dDay = new Date(d.getFullYear(), d.getMonth(), d.getDate());
    const nDay = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const diffDays = Math.round((nDay.getTime() - dDay.getTime()) / 86400000);

    if (diffDays === 0) return $LL.chat.today();
    if (diffDays === 1) return $LL.chat.yesterday();
    if (diffDays < 7) return d.toLocaleDateString('en', { weekday: 'long' });
    return d.toLocaleDateString('en', { month: 'short', day: 'numeric' });
  }

  function handleContextMenu(e: MouseEvent, sessionId: string) {
    e.preventDefault();
    contextMenuSessionId = sessionId;
    contextMenuPosition = { x: e.clientX, y: e.clientY };
  }

  function closeContextMenu(e?: MouseEvent) {
    if (e) {
      const menu = document.querySelector('.context-menu');
      if (menu && !menu.contains(e.target as Node)) {
        contextMenuSessionId = null;
      }
    } else {
      contextMenuSessionId = null;
    }
  }
</script>

<svelte:window onclick={(e) => closeContextMenu(e)} />

<aside class="sidebar">
  <div class="sidebar__header">
    <span class="sidebar__title">Chats</span>
    <div class="sidebar__header-actions">
      <button class="sidebar__header-btn" onclick={newChat}         aria-label={$LL.aria.newChat()}>
        <MessageSquare size={16} strokeWidth={1.5} />
      </button>
    </div>
  </div>

  <div class="sidebar__list">
    {#each sessions as session (session.id)}
      <div
        class="sidebar__item"
        class:sidebar__item--active={$chatStore.activeSessionId === session.id}
        class:sidebar__item--pinned={session.isPinned}
        class:sidebar__item--archived={session.isArchived}
        class:sidebar__item--selected={$chatStore.selectMode && $chatStore.selectedSessionIds.has(session.id)}
        role="button"
        tabindex="0"
        onclick={() => selectSession(session.id)}
        onkeydown={(e: KeyboardEvent) => { if (e.key === 'Enter') selectSession(session.id); }}
        oncontextmenu={(e) => handleContextMenu(e, session.id)}
      >
        {#if $chatStore.selectMode}
          <span class="sidebar__checkbox">
            {#if $chatStore.selectedSessionIds.has(session.id)}
              <CheckSquare size={14} />
            {:else}
              <Square size={14} />
            {/if}
          </span>
        {/if}

        <span class="sidebar__item-icon">
          {#if session.isPinned}
            <Pin size={12} />
          {:else}
            <MessageSquare size={12} />
          {/if}
        </span>

        <div class="sidebar__item-body">
          {#if editingSessionId === session.id}
            <input
              class="sidebar__rename-input"
              type="text"
              bind:value={editTitle}
              onkeydown={(e: KeyboardEvent) => {
                if (e.key === 'Enter') saveRename();
                if (e.key === 'Escape') cancelRename();
              }}
              onblur={saveRename}
              use:autofocus
            />
          {:else}
            <span class="sidebar__item-title">{session.title}</span>
            <span class="sidebar__item-date">{formatDate(session.createdAt)}</span>
          {/if}
        </div>

        <button
          class="sidebar__more"
          onclick={(e) => { e.stopPropagation(); handleContextMenu(e, session.id); }}
          aria-label={$LL.aria.moreActions()}
        >
          <MoreHorizontal size={14} />
        </button>
      </div>
    {:else}
      <div class="sidebar__empty">
        {#if showArchived}
          <span>{$LL.noArchivedChats()}</span>
        {:else}
          <span>{$LL.chat.noSessions()}</span>
        {/if}
      </div>
    {/each}
  </div>

  <div class="sidebar__footer">
    <label class="sidebar__toggle-archive">
      <input type="checkbox" bind:checked={showArchived} />
      <span>{$LL.showArchived()}</span>
    </label>
    {#if $devMode}
      <button class="sidebar__logs-link" onclick={() => goto('/logs')}>
        <FileText size={14} />
        <span>{$LL.viewLogs()}</span>
      </button>
    {/if}
  </div>
</aside>

{#if contextMenuSessionId}
  {@const session = $chatStore.sessions.find((s) => s.id === contextMenuSessionId)}
  {#if session}
    <div
      class="context-menu"
      style="left: {contextMenuPosition.x}px; top: {contextMenuPosition.y}px;"
      role="menu"
    >
      <button class="context-menu__item" onclick={() => { startRename(session); closeContextMenu(); }}>
        {$LL.rename()}
      </button>
      <button class="context-menu__item" onclick={() => { handlePin(session); closeContextMenu(); }}>
        {session.isPinned ? $LL.unpin() : $LL.pin()}
      </button>
      <button class="context-menu__item" onclick={() => { handleArchive(session); closeContextMenu(); }}>
        {session.isArchived ? $LL.unarchive() : $LL.archive()}
      </button>
            <button class="context-menu__item" onclick={() => { handleCopyConversation(session); closeContextMenu(); }}>
        {$LL.copy()}
      </button>
      <button class="context-menu__item" onclick={() => { handleExport(session); closeContextMenu(); }}>
        {$LL.export()}
      </button>
      <hr class="context-menu__divider" />
      <button class="context-menu__item context-menu__item--danger" onclick={() => { handleDelete(session); closeContextMenu(); }}>
        {$LL.delete()}
      </button>
    </div>
  {/if}
{/if}

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    border-right: 1px solid var(--border);
    background: var(--surface);
    flex-shrink: 0;
  }

  .sidebar__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 14px 10px;
  }

  .sidebar__title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .sidebar__header-btn {
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
  }

  .sidebar__header-btn:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .sidebar__list {
    flex: 1;
    overflow-y: auto;
    padding: 4px 8px;
  }

  .sidebar__item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 10px;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: var(--text);
    cursor: pointer;
    text-align: left;
    font-family: inherit;
    font-size: 13px;
    transition: background 0.12s;
  }

  .sidebar__item:hover {
    background: var(--surface);
  }

  .sidebar__item--active,
  .sidebar__item--active:hover {
    background: color-mix(in srgb, var(--primary) 15%, transparent);
  }

  .sidebar__item--pinned {
    background: color-mix(in srgb, var(--primary) 5%, transparent);
  }

  .sidebar__item--archived {
    opacity: 0.55;
  }

  .sidebar__item--selected {
    background: color-mix(in srgb, var(--primary) 12%, transparent);
  }

  .sidebar__item:focus-visible {
    outline: 2px solid var(--primary);
    outline-offset: -2px;
  }

  .sidebar__checkbox {
    display: flex;
    align-items: center;
    color: var(--primary);
    flex-shrink: 0;
  }

  .sidebar__item-icon {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    color: var(--text-dim);
  }

  .sidebar__item-body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .sidebar__item-title {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-weight: 500;
  }

  .sidebar__item-date {
    font-size: 11px;
    color: var(--text-dim);
  }

  .sidebar__rename-input {
    width: 100%;
    padding: 3px 6px;
    border: 1px solid var(--primary);
    border-radius: 4px;
    background: var(--bg);
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    outline: none;
  }

  .sidebar__more {
    display: none;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border: none;
    border-radius: 4px;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    flex-shrink: 0;
  }

  .sidebar__item:hover .sidebar__more {
    display: flex;
  }

  .sidebar__more:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .sidebar__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px 16px;
    font-size: 12px;
    color: var(--text-dim);
  }

  .sidebar__footer {
    padding: 8px 14px 12px;
    border-top: 1px solid var(--border);
  }

  .sidebar__toggle-archive {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-muted);
    cursor: pointer;
  }

  .sidebar__toggle-archive input {
    accent-color: var(--primary);
  }

  .sidebar__logs-link {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 8px;
    padding: 6px 0;
    font-size: 12px;
    color: var(--primary);
    background: none;
    border: none;
    cursor: pointer;
    font-family: inherit;
  }

  .sidebar__logs-link:hover {
    color: var(--primary-hover);
  }

  .context-menu {
    position: fixed;
    z-index: 1000;
    min-width: 160px;
    padding: 6px;
    border: 1px solid var(--border);
    border-radius: 10px;
    background: var(--bg);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.18);
  }

  .context-menu__item {
    display: block;
    width: 100%;
    padding: 7px 12px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text);
    font-family: inherit;
    font-size: 12px;
    text-align: left;
    cursor: pointer;
  }

  .context-menu__item:hover {
    background: var(--surface-hover);
  }

  .context-menu__item--danger {
    color: var(--error);
  }

  .context-menu__item--danger:hover {
    background: color-mix(in srgb, var(--error) 10%, transparent);
  }

  .context-menu__divider {
    margin: 4px 0;
    border: none;
    border-top: 1px solid var(--border);
  }
</style>
