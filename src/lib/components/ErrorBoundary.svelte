<script lang="ts">
  import { onMount } from 'svelte';
  import { CircleAlert, RefreshCw } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    name?: string;
    children?: import('svelte').Snippet;
  }

  let { name = 'component', children }: Props = $props();

  let error: Error | null = $state(null);

  $effect(() => {
    window.addEventListener('error', onGlobalError);
    window.addEventListener('unhandledrejection', onPromiseError);
    return () => {
      window.removeEventListener('error', onGlobalError);
      window.removeEventListener('unhandledrejection', onPromiseError);
    };
  });

  function onGlobalError(event: ErrorEvent) {
    if (!error) {
      error = event.error || new Error(event.message);
      event.preventDefault();
    }
  }

  function onPromiseError(event: PromiseRejectionEvent) {
    if (!error) {
      error = event.reason instanceof Error ? event.reason : new Error(String(event.reason));
      event.preventDefault();
    }
  }

  function reset() {
    error = null;
  }

  function reload() {
    window.location.reload();
  }
</script>

{#snippet fallback()}
  <div class="error-boundary">
    <div class="error-boundary__card">
      <div class="error-boundary__icon">
        <CircleAlert strokeWidth={1.5} size={28} />
      </div>
      <h3 class="error-boundary__title">{$LL.somethingWentWrong()}</h3>
      {#if name}
        <p class="error-boundary__detail">{$LL.errorBoundary.errorInName({ name })}</p>
      {/if}
      <pre class="error-boundary__message">{error?.message ?? $LL.errorBoundary.unknownError()}</pre>
      <div class="error-boundary__actions">
        <button class="error-boundary__btn error-boundary__btn--primary" onclick={reset}>
          <RefreshCw strokeWidth={1.5} size={14} />
          {$LL.errorBoundary.tryAgain()}
        </button>
        <button class="error-boundary__btn" onclick={reload}>
          {$LL.errorBoundary.reloadPage()}
        </button>
      </div>
    </div>
  </div>
{/snippet}

{#if error}
  {@render fallback()}
{:else if children}
  {@render children()}
{/if}

<style>
  .error-boundary {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 40px 20px;
  }

  .error-boundary__card {
    text-align: center;
    max-width: 360px;
  }

  .error-boundary__icon {
    color: var(--error);
    margin-bottom: 12px;
  }

  .error-boundary__title {
    margin: 0 0 4px;
    font-size: 16px;
    font-weight: 600;
    color: var(--text);
  }

  .error-boundary__detail {
    margin: 0 0 12px;
    font-size: 12px;
    color: var(--text-muted);
  }

  .error-boundary__message {
    padding: 10px 14px;
    margin: 0 0 16px;
    background: var(--code-bg);
    border-radius: 8px;
    font-size: 12px;
    color: var(--text-muted);
    text-align: left;
    white-space: pre-wrap;
    word-break: break-word;
    max-height: 120px;
    overflow-y: auto;
  }

  .error-boundary__actions {
    display: flex;
    gap: 8px;
    justify-content: center;
  }

  .error-boundary__btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    cursor: pointer;
  }

  .error-boundary__btn:hover {
    background: var(--surface-hover);
  }

  .error-boundary__btn--primary {
    background: var(--primary);
    color: white;
    border-color: var(--primary);
  }

  .error-boundary__btn--primary:hover {
    background: var(--primary-hover);
  }
</style>
