<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { chatStore } from '$lib/stores/chat';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    model: string;
    maxTokens: number;
    currentTokens: number;
  }

  let { model, maxTokens, currentTokens }: Props = $props();

  let usagePercent = $derived(
    maxTokens > 0 ? Math.round((currentTokens / maxTokens) * 100) : 0,
  );

  let barColor = $derived(
    usagePercent > 95
      ? 'var(--error)'
      : usagePercent > 80
        ? 'var(--warning)'
        : usagePercent > 50
          ? 'var(--accent)'
          : 'var(--success)',
  );

  let pulse = $derived(usagePercent > 90);

  const streaming = $derived($chatStore.streamingStatus === 'streaming');

  let isExpanded = $state(false);
</script>

<div
  class="context-bar"
  class:context-bar--expanded={isExpanded}
  role="meter"
  aria-label={$LL.aria.contextWindowUsage()}
  aria-valuenow={currentTokens}
  aria-valuemin={0}
  aria-valuemax={maxTokens}
  onmouseenter={() => (isExpanded = true)}
  onmouseleave={() => (isExpanded = false)}
>
  <div class="context-bar__gauge">
    <div class="context-bar__track">
      <div
        class="context-bar__fill"
        class:context-bar__fill--pulse={pulse && streaming}
        style="width:{Math.min(usagePercent, 100)}%; background:{barColor}"
      ></div>
    </div>
    <span class="context-bar__label">
      {currentTokens.toLocaleString()} / {maxTokens.toLocaleString()}
      <span class="context-bar__percent">({usagePercent}%)</span>
    </span>
  </div>

  {#if isExpanded}
    <div class="context-bar__detail">
      <span class="context-bar__detail-label">{$LL.contextBar.label()}</span>
      {#if usagePercent > 80}
        <span class="context-bar__detail-value context-bar__detail-value--warning">
          {$LL.contextBar.warning()}
        </span>
      {:else}
        <span class="context-bar__detail-value">{currentTokens.toLocaleString()} {$LL.contextBar.tokensUsed()}</span>
      {/if}
    </div>
  {/if}
</div>

<style>
  .context-bar {
    padding: 2px 16px 6px;
    transition: padding 0.2s;
  }

  .context-bar--expanded {
    padding: 6px 16px 10px;
  }

  .context-bar__gauge {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .context-bar__track {
    flex: 1;
    height: 4px;
    border-radius: 2px;
    background: var(--surface-hover);
    overflow: hidden;
  }

  .context-bar__fill {
    height: 100%;
    border-radius: 2px;
    transition: width 0.3s ease, background 0.3s;
  }

  .context-bar__fill--pulse {
    animation: context-pulse 1.5s ease-in-out infinite;
  }

  @keyframes context-pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  .context-bar__label {
    flex-shrink: 0;
    font-size: 10px;
    color: var(--text-dim);
    white-space: nowrap;
  }

  .context-bar__percent {
    opacity: 0.6;
  }

  .context-bar__detail {
    margin-top: 4px;
    font-size: 11px;
    color: var(--text-muted);
  }

  .context-bar__detail-label {
    font-weight: 600;
    margin-right: 6px;
  }

  .context-bar__detail-value--warning {
    color: var(--warning);
  }
</style>
