<script lang="ts">
  import { Folder, Plus, Trash2, CheckCircle } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    workspaces: string[];
    activeWorkspace: string | null;
    onAdd: () => void;
    onRemove: (path: string) => void;
    onSelect: (path: string) => void;
  }

  let { workspaces, activeWorkspace, onAdd, onRemove, onSelect }: Props = $props();
</script>

<div class="workspace-selector">
  {#if workspaces.length === 0}
    <div class="workspace-empty">
      <Folder size={24} strokeWidth={1.5} />
      <p>{$LL.workspaces.noWorkspaces()}</p>
      <p class="workspace-cta">{$LL.workspaces.addCTA()}</p>
    </div>
  {:else}
    <div class="workspace-list">
      {#each workspaces as path}
        <div
          class="workspace-item"
          class:workspace-item--active={path === activeWorkspace}
          role="button"
          tabindex="0"
          onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); onSelect(path); } }}
        >
          <button class="workspace-item__select" onclick={() => onSelect(path)}>
            <Folder size={15} strokeWidth={1.5} />
            <span class="workspace-item__path">{path}</span>
          </button>
          {#if path === activeWorkspace}
            <span class="workspace-item__badge">
              <CheckCircle size={12} strokeWidth={1.5} />
              {$LL.workspaces.active()}
            </span>
          {/if}
          <button
            class="workspace-item__remove"
            onclick={() => onRemove(path)}
            aria-label={$LL.workspaces.remove() as string}
          >
            <Trash2 size={13} strokeWidth={1.5} />
          </button>
        </div>
      {/each}
    </div>
  {/if}

  <button class="workspace-add-btn" onclick={onAdd}>
    <Plus size={14} strokeWidth={1.5} />
    {$LL.workspaces.add()}
  </button>
</div>

<style>
  .workspace-selector {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .workspace-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 24px 16px;
    text-align: center;
    color: var(--text-muted);
  }

  .workspace-empty p {
    margin: 0;
    font-size: 13px;
  }

  .workspace-cta {
    font-size: 12px;
    color: var(--text-dim);
  }

  .workspace-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .workspace-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 10px;
    border-radius: 7px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .workspace-item--active {
    border-color: var(--primary);
  }

  .workspace-item__select {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    border: none;
    background: transparent;
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
    text-align: left;
    min-width: 0;
    padding: 0;
  }

  .workspace-item__path {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 12px;
  }

  .workspace-item__badge {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    font-size: 10px;
    font-weight: 600;
    color: var(--primary);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    white-space: nowrap;
  }

  .workspace-item__remove {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border: none;
    border-radius: 4px;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    flex-shrink: 0;
  }

  .workspace-item__remove:hover {
    background: color-mix(in srgb, var(--error) 10%, transparent);
    color: var(--error);
  }

  .workspace-add-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    border: 1px dashed var(--border);
    border-radius: 8px;
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
    width: 100%;
    justify-content: center;
  }

  .workspace-add-btn:hover {
    border-color: var(--primary);
    color: var(--primary);
  }
</style>
