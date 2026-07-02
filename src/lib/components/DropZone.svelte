<script lang="ts">
  import { Upload } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    active: boolean;
  }

  let { active }: Props = $props();
</script>

<div class="dropzone" class:dropzone--active={active} role="presentation">
  <div class="dropzone__content">
    <Upload size={32} strokeWidth={1.5} />
    <span class="dropzone__text">{$LL.dropZone.dropFiles()}</span>
    <span class="dropzone__hint">{$LL.dropZone.hint()}</span>
  </div>
</div>

<style>
  .dropzone {
    position: absolute;
    inset: 0;
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in srgb, var(--primary) 15%, var(--bg) 85%);
    backdrop-filter: blur(4px);
    border: 2px dashed var(--primary);
    border-radius: 0;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.15s ease;
  }

  .dropzone--active {
    opacity: 1;
  }

  .dropzone__content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    color: var(--primary);
  }

  .dropzone__text {
    font-size: 16px;
    font-weight: 600;
  }

  .dropzone__hint {
    font-size: 12px;
    color: var(--text-muted);
  }
</style>
