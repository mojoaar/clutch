<script lang="ts">
  interface Props {
    label: string;
    shortcut?: string;
    description?: string;
  }

  let { label, shortcut, description }: Props = $props();
</script>

<kbd class="kb-shortcut" aria-label={label} data-shortcut={shortcut}>
  {#if shortcut}
    {shortcut}
  {/if}
  {#if description}
    <span class="kb-shortcut__desc">{description}</span>
  {/if}
</kbd>

<style>
  .kb-shortcut {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 500;
    font-family: var(--font-mono, "JetBrains Mono", monospace);
    background: var(--surface-hover);
    color: var(--text-muted);
    border: 1px solid var(--border);
  }

  .kb-shortcut__desc {
    font-family: var(--font-sans, system-ui, sans-serif);
    font-size: 11px;
  }
</style>
