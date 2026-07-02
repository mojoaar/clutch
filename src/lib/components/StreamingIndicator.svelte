<script lang="ts">
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    message?: string;
  }

  let { message = 'AI is thinking…' }: Props = $props();

  let tokens = $state(0);
  let interval: ReturnType<typeof setInterval>;

  $effect(() => {
    interval = setInterval(() => {
      tokens += Math.floor(Math.random() * 8) + 1;
    }, 400);
    return () => clearInterval(interval);
  });
</script>

<div class="streaming-indicator" role="status" aria-live="polite" aria-label={message}>
  <span class="streaming-indicator__cursor"></span>
  <span class="streaming-indicator__label">{message}</span>
  <span class="streaming-indicator__tokens">{tokens} {$LL.streamingTokens()}</span>
</div>

<style>
  .streaming-indicator {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 4px 0;
    font-size: 12px;
    color: var(--text-muted);
  }

  .streaming-indicator__cursor {
    display: inline-block;
    width: 2px;
    height: 14px;
    background: var(--primary);
    animation: blink-cursor 0.8s steps(1) infinite;
  }

  @keyframes blink-cursor {
    0%, 100% { opacity: 1; }
    50% { opacity: 0; }
  }

  .streaming-indicator__tokens {
    font-variant-numeric: tabular-nums;
  }
</style>
