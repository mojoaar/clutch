<script lang="ts">
  import { chatStore } from '$lib/stores/chat';
  import { sendMessage } from '$lib/services/api';
  import { RefreshCw } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    messageId: string;
    sessionId: string;
    provider: string;
    model: string;
    systemPrompt?: string;
  }

  let {
    messageId,
    sessionId,
    provider,
    model,
    systemPrompt,
  }: Props = $props();

  let isRetrying = $state(false);

  const streamingState = $derived($chatStore.streamingStatus);
  const isInterrupted = $derived(streamingState === 'interrupted');

  async function retryFromScratch() {
    isRetrying = true;
    try {
      chatStore.markMessageDeleted(messageId);

      const currentMessages = $chatStore.messages.filter(
        (m) => !m.isDeleted && m.sessionId === sessionId && m.id !== messageId,
      );

      chatStore.setStreamingStatus('idle');

      await new Promise((resolve) => setTimeout(resolve, 500));

      const lastUserMsg = [...currentMessages].reverse().find((m) => m.role === 'user');
      if (lastUserMsg) {
        await sendMessage(sessionId, lastUserMsg.content, provider, model, currentMessages.slice(0, -1), systemPrompt);
      }
    } finally {
      isRetrying = false;
    }
  }

  function dismiss() {
    chatStore.setStreamingStatus('idle');
  }
</script>

{#if isInterrupted}
  <div class="interrupt-banner" role="alert">
    <div class="interrupt-banner__content">
      <span class="interrupt-banner__text">{$LL.chat.responseInterrupted()}</span>
      <div class="interrupt-banner__actions">
        <button
          class="interrupt-banner__btn interrupt-banner__btn--retry"
          onclick={retryFromScratch}
          disabled={isRetrying}
          aria-label={$LL.aria.retryFromScratch()}
        >
          {#if isRetrying}
            {$LL.retrying()}
          {:else}
            <RefreshCw size={13} strokeWidth={1.5} />
            {$LL.retry()}
          {/if}
        </button>
        <button
          class="interrupt-banner__btn interrupt-banner__btn--dismiss"
          onclick={dismiss}
          aria-label={$LL.dismiss()}
        >
          {$LL.dismiss()}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .interrupt-banner {
    margin: 4px 20px 8px;
    padding: 8px 14px;
    border-radius: 8px;
    border: 1px solid var(--warning);
    background: color-mix(in srgb, var(--warning) 8%, var(--surface));
  }

  .interrupt-banner__content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .interrupt-banner__text {
    font-size: 13px;
    font-weight: 600;
    color: var(--warning);
  }

  .interrupt-banner__actions {
    display: flex;
    gap: 8px;
  }

  .interrupt-banner__btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 10px;
    border-radius: 6px;
    border: none;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
  }

  .interrupt-banner__btn--retry {
    background: var(--warning);
    color: var(--bg);
  }

  .interrupt-banner__btn--retry:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .interrupt-banner__btn--dismiss {
    background: var(--surface-hover);
    color: var(--text-muted);
  }

  .interrupt-banner__btn--dismiss:hover {
    color: var(--text);
  }
</style>
