<script lang="ts">
  import { Bot } from '@lucide/svelte';
  import { PROVIDER_COLORS } from '$lib/utils/provider-colors';

  interface Props {
    provider?: string;
    size?: number;
  }

  let { provider, size = 28 }: Props = $props();

  let color = $derived(
    provider ? (PROVIDER_COLORS[provider] ?? 'var(--primary)') : 'var(--primary)',
  );
</script>

<div class="ai-avatar" style="width: {size}px; height: {size}px">
  <Bot class="ai-avatar__icon" size={Math.floor(size * 0.7)} strokeWidth={1.5} />
  <div class="ai-avatar__dot" style="background: {color}"></div>
</div>

<style>
  .ai-avatar {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--surface);
    color: var(--text);
    flex-shrink: 0;
  }

  :global(.ai-avatar__icon) {
    color: var(--text-dim);
  }

  .ai-avatar__dot {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    border: 2px solid var(--bg);
  }
</style>
