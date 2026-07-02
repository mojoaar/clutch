<script lang="ts">
  import { PROVIDERS, type ProviderConfig } from '$lib/services/providers';
  import {
    getModelsCached,
    refreshModels,
    getDefaultModels,
    groupModelsByCategory,
    type ModelInfo,
  } from '$lib/services/models';
  import { addToast } from '$lib/stores/toast';
  import { RefreshCw, Clock, HardDrive } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    apiKeys: Record<string, string>;
  }

  let { apiKeys }: Props = $props();

  let providers = $state(Object.values(PROVIDERS));
  let modelsByProvider = $state<Record<string, ModelInfo[]>>({});
  let loading = $state<Record<string, boolean>>({});
  let expanded: Record<string, boolean> = $state({});

  async function loadProviderModels(provider: ProviderConfig) {
    const key = apiKeys[provider.id] || '';
    loading[provider.id] = true;
    try {
      if (key) {
        modelsByProvider[provider.id] = await getModelsCached(provider.id, key);
      } else {
        modelsByProvider[provider.id] = getDefaultModels(provider.id);
      }
    } catch {
      modelsByProvider[provider.id] = getDefaultModels(provider.id);
    } finally {
      loading[provider.id] = false;
    }
  }

  async function handleRefresh(providerId: string) {
    const key = apiKeys[providerId] || '';
    if (!key) return;
    loading[providerId] = true;
    try {
      modelsByProvider[providerId] = await refreshModels(providerId, key);
      addToast(`Models refreshed`, 'info');
    } catch {
      addToast(`Failed to refresh models`, 'error');
    } finally {
      loading[providerId] = false;
    }
  }

  async function handleRefreshAll() {
    for (const p of providers) {
      const key = apiKeys[p.id] || '';
      if (!key) continue;
      loading[p.id] = true;
      try {
        modelsByProvider[p.id] = await refreshModels(p.id, key);
      } catch {
        // skip
      } finally {
        loading[p.id] = false;
      }
    }
    addToast($LL.modelCache.refreshed(), 'info');
  }

  function toggleExpand(id: string) {
    expanded[id] = !expanded[id];
  }

  $effect(() => {
    for (const p of providers) {
      loadProviderModels(p);
    }
  });
</script>

<div class="model-selector" role="region" aria-label={$LL.aria.availableModels()}>
  <div class="models-header">
    <div>
      <h3 class="models-title">{$LL.models()}</h3>
      <p class="models-subtitle">{$LL.modelCache.cachedHint()}</p>
    </div>
    <button class="models-refresh-btn" onclick={handleRefreshAll} aria-label={$LL.refresh()}>
      <RefreshCw size={14} strokeWidth={1.5} />
      {$LL.modelCache.refreshAll()}
    </button>
  </div>

  {#each providers as provider}
    <div class="model-provider">
      <button class="model-provider__header" onclick={() => toggleExpand(provider.id)}>
        <span class="model-provider__name">{provider.name}</span>
        <span class="model-provider__count">
          {$LL.modelCache.modelsCount({ count: modelsByProvider[provider.id]?.length ?? 0 })}
        </span>
        <span class:model-provider__arrow={true} class:model-provider__arrow--open={expanded[provider.id]}>▾</span>
      </button>

      {#if expanded[provider.id]}
        <div class="model-provider__body">
          {#if loading[provider.id]}
            <div class="model-loading">{$LL.loading()}</div>
          {:else if modelsByProvider[provider.id]?.length}
            {#each [...groupModelsByCategory(modelsByProvider[provider.id] || [])] as [category, categoryModels]}
              <div class="model-category">
                <span class="model-category__name">{category}</span>
                <div class="model-category__list">
                  {#each categoryModels as model}
                    <div class="model-item">
                      <HardDrive size={12} strokeWidth={1.5} />
                      <span class="model-item__name">{model.name}</span>
                      {#if model.context_length}
                        <span class="model-item__ctx">{model.context_length.toLocaleString()} ctx</span>
                      {/if}
                    </div>
                  {/each}
                </div>
              </div>
            {/each}
          {:else}
            <div class="model-empty">{$LL.modelCache.noModels()}</div>
          {/if}

          <div class="model-footer">
            <span class="model-footer__cache">
              <Clock size={12} strokeWidth={1.5} />
              {$LL.modelCache.ttl()}
            </span>
            <button
              class="model-footer__refresh"
              onclick={() => handleRefresh(provider.id)}
              disabled={loading[provider.id]}
            >
              <RefreshCw size={12} strokeWidth={1.5} />
              {$LL.modelCache.refresh()}
            </button>
          </div>
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .model-selector {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .models-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }

  .models-title {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
  }

  .models-subtitle {
    margin: 2px 0 0;
    font-size: 12px;
    color: var(--text-dim);
  }

  .models-refresh-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 6px 12px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: var(--bg);
    color: var(--text);
    font-family: inherit;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .models-refresh-btn:hover {
    background: var(--surface-hover);
  }

  .model-provider {
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
    background: var(--surface);
  }

  .model-provider__header {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 10px 12px;
    border: none;
    background: transparent;
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
    text-align: left;
  }

  .model-provider__header:hover {
    background: var(--surface-hover);
  }

  .model-provider__name {
    font-weight: 600;
  }

  .model-provider__count {
    font-size: 11px;
    color: var(--text-dim);
    margin-left: auto;
  }

  .model-provider__arrow {
    font-size: 10px;
    color: var(--text-dim);
    transition: transform 0.15s;
  }

  .model-provider__arrow--open {
    transform: rotate(180deg);
  }

  .model-provider__body {
    padding: 8px 12px 0;
    border-top: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .model-category__name {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    margin-bottom: 4px;
    display: block;
  }

  .model-category__list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .model-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 6px;
    border-radius: 5px;
    font-size: 12px;
  }

  .model-item__name {
    color: var(--text);
  }

  .model-item__ctx {
    margin-left: auto;
    font-size: 10px;
    color: var(--text-dim);
    white-space: nowrap;
  }

  .model-loading,
  .model-empty {
    font-size: 12px;
    color: var(--text-dim);
    padding: 8px 0;
  }

  .model-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0;
    border-top: 1px solid var(--border);
  }

  .model-footer__cache {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    color: var(--text-dim);
  }

  .model-footer__refresh {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    border: none;
    border-radius: 5px;
    background: transparent;
    color: var(--primary);
    font-family: inherit;
    font-size: 11px;
    cursor: pointer;
  }

  .model-footer__refresh:hover {
    background: color-mix(in srgb, var(--primary) 10%, transparent);
  }

  .model-footer__refresh:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
