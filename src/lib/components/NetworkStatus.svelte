<script lang="ts">
  import { network, type NetworkStatus } from '$lib/services/network';
  import { onMount, onDestroy } from 'svelte';
  import { goto } from '$app/navigation';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    provider?: string;
    model?: string;
    configured?: boolean;
  }

  let { provider, model, configured = false }: Props = $props();

  let status = $state<NetworkStatus>('online');
  let queueSize = $state(0);
  let showClearConfirm = $state(false);

  onMount(() => {
    network.startMonitoring();
    const unsubStatus = network.status.subscribe((s) => {
      status = s;
    });
    const unsubQueue = network.queueSize.subscribe((s) => {
      queueSize = s;
    });
    return () => {
      network.stopMonitoring();
      unsubStatus();
      unsubQueue();
    };
  });

  const colors = $derived<Record<NetworkStatus, string>>({
    online: configured ? 'var(--c-success)' : '#f59e0b',
    degraded: '#f59e0b',
    offline: 'var(--c-error)',
  });

  const labels = $derived<Record<NetworkStatus, string>>({
    online: configured ? $LL.networkStatus.connected() : $LL.networkStatus.noApiKey(),
    degraded: $LL.networkStatus.slowConnection(),
    offline: $LL.networkStatus.offline(),
  });

  function handleRetry() {
    network.retryAll();
  }

  function handleClear() {
    if (!showClearConfirm) {
      showClearConfirm = true;
      setTimeout(() => (showClearConfirm = false), 5000);
      return;
    }
    network.clearQueue();
    showClearConfirm = false;
  }
</script>

<div class="network-status" role="status" aria-label={labels[status]}>
  <span class="dot" style="background-color: {colors[status]};"></span>
  <span class="label">
    {#if !configured && status === 'online'}
      <button class="link" onclick={() => goto('/settings?tab=providers')}>{$LL.networkStatus.configureProvider()}</button>
    {:else}
      {labels[status]}
      {#if provider && model}
        · {provider} / {model}
      {/if}
    {/if}
  </span>
  {#if queueSize > 0}
    <span class="queue-badge">{$LL.networkStatus.messagesQueued({ count: queueSize })}</span>
    <button class="queue-btn" onclick={handleRetry} aria-label={$LL.networkStatus.retryQueue()}>
      {$LL.retry()}
    </button>
    <button
      class="queue-btn queue-btn--danger"
      onclick={handleClear}
      aria-label={showClearConfirm ? '' : $LL.networkStatus.clearQueue()}
    >
      {showClearConfirm ? $LL.confirm() : $LL.clear()}
    </button>
  {/if}
</div>

<style>
  .network-status {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 11px;
    color: var(--c-text-muted);
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
    animation: pulse-dot 2s ease-in-out infinite;
  }

  @keyframes pulse-dot {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(1.3); }
  }

  .label {
    white-space: nowrap;
  }

  .link {
    border: none;
    background: none;
    color: inherit;
    font-family: inherit;
    font-size: inherit;
    cursor: pointer;
    text-decoration: underline;
    padding: 0;
  }

  .link:hover {
    color: var(--text);
  }

  .queue-badge {
    background: var(--c-primary);
    color: white;
    padding: 1px 6px;
    border-radius: 8px;
    font-size: 10px;
    font-weight: 600;
    white-space: nowrap;
  }

  .queue-btn {
    border: none;
    background: var(--c-surface-hover);
    color: var(--c-text-muted);
    font-family: inherit;
    font-size: 10px;
    cursor: pointer;
    padding: 1px 6px;
    border-radius: 4px;
    white-space: nowrap;
  }

  .queue-btn:hover {
    color: var(--c-text);
    background: var(--c-surface-active);
  }

  .queue-btn--danger {
    color: var(--c-error);
  }

  .queue-btn--danger:hover {
    background: var(--c-error);
    color: white;
  }
</style>
