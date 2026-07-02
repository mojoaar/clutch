<script lang="ts">
  import { formatFileSize } from '$lib/utils/file-utils';
  import { Image, FileText, X, LoaderCircle } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    name: string;
    size: number;
    type: string;
    progress?: number;
    onRemove?: () => void;
  }

  let { name, size, type: fileType, progress, onRemove }: Props = $props();

  let isImage = $derived(fileType.startsWith('image/'));
  let ext = $derived(name.split('.').pop()?.toUpperCase() ?? '');
  let fileIcon = $derived(isImage ? Image : FileText);
</script>

<div class="file-preview">
  {#if isImage}
    <div class="file-preview__thumb file-preview__thumb--image">
      <Image size={20} strokeWidth={1.5} />
    </div>
  {:else}
    <div class="file-preview__thumb">
      <span class="file-preview__ext">{ext}</span>
    </div>
  {/if}

  <div class="file-preview__info">
    <span class="file-preview__name">{name}</span>
    <span class="file-preview__meta">
      {formatFileSize(size)}
    </span>
  </div>

  {#if progress !== undefined && progress < 100}
    <div class="file-preview__loader">
      <LoaderCircle size={16} strokeWidth={1.5} class="spinner" />
    </div>
  {/if}

  {#if onRemove}
    <button class="file-preview__remove" onclick={onRemove} aria-label={$LL.aria.removeFile()}>
      <X size={14} strokeWidth={2} />
    </button>
  {/if}
</div>

<style>
  .file-preview {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: 8px;
    border: 1px solid var(--border);
    background: var(--surface);
  }

  .file-preview__thumb {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 6px;
    background: var(--surface-hover);
    flex-shrink: 0;
  }

  .file-preview__ext {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-dim);
    letter-spacing: 0.03em;
  }

  .file-preview__info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .file-preview__name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-preview__meta {
    font-size: 11px;
    color: var(--text-dim);
  }

  .file-preview__loader {
    display: flex;
    align-items: center;
    color: var(--primary);
  }

  :global(.spinner) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .file-preview__remove {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    flex-shrink: 0;
  }

  .file-preview__remove:hover {
    background: var(--surface-hover);
    color: var(--error);
  }
</style>
