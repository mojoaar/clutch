<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import LL from '$lib/i18n/i18n-svelte';
  import { toasts, dismissToast } from '$lib/stores/toast';
  import { CircleAlert, CheckCircle, AlertTriangle, Info, X } from '@lucide/svelte';

  const iconMap: Record<string, typeof CircleAlert> = {
    error: CircleAlert,
    warning: AlertTriangle,
    success: CheckCircle,
    info: Info,
  };
</script>

{#if $toasts.length > 0}
  <div class="toast-container" role="status" aria-live="polite">
    {#each $toasts as toast (toast.id)}
      <div
        class="toast toast--{toast.variant}"
        role="alert"
        transition:fly={{ y: -20, duration: 200 }}
      >
        <div class="toast__icon">
          <svelte:component this={iconMap[toast.variant]} strokeWidth={1.5} size={18} />
        </div>
        <span class="toast__message">{toast.message}</span>
        {#if toast.action}
          <button class="toast__action" onclick={toast.action.onClick}>
            {toast.action.label}
          </button>
        {/if}
        {#if toast.dismissible}
          <button
            class="toast__close"
            aria-label={$LL.dismiss()}
            onclick={() => dismissToast(toast.id)}
          >
            <X strokeWidth={1.5} size={14} />
          </button>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .toast-container {
    position: fixed;
    top: 12px;
    right: 12px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 380px;
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 13px;
    line-height: 1.4;
    pointer-events: auto;
    box-shadow: 0 4px 16px var(--shadow);
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
  }

  .toast--error {
    border-color: var(--error);
    background: color-mix(in srgb, var(--error) 8%, var(--surface));
  }

  .toast--warning {
    border-color: var(--warning);
    background: color-mix(in srgb, var(--warning) 8%, var(--surface));
  }

  .toast--success {
    border-color: var(--success);
    background: color-mix(in srgb, var(--success) 8%, var(--surface));
  }

  .toast--info {
    border-color: var(--primary);
    background: color-mix(in srgb, var(--primary) 8%, var(--surface));
  }

  .toast__icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .toast--error .toast__icon {
    color: var(--error);
  }

  .toast--warning .toast__icon {
    color: var(--warning);
  }

  .toast--success .toast__icon {
    color: var(--success);
  }

  .toast--info .toast__icon {
    color: var(--primary);
  }

  .toast__message {
    flex: 1;
    min-width: 0;
  }

  .toast__action {
    flex-shrink: 0;
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    border: none;
    cursor: pointer;
    background: var(--primary);
    color: white;
  }

  .toast__close {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .toast__close:hover {
    background: var(--surface-hover);
    color: var(--text);
  }
</style>
