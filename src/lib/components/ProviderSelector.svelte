<script lang="ts">
  import type { ProviderConfig } from '$lib/services/providers';
  import { getModelsCached, refreshModels, getDefaultModels, type ModelInfo } from '$lib/services/models';
  import { testConnection, getSetting } from '$lib/services/settings';
  import { addToast } from '$lib/stores/toast';
  import { Eye, EyeOff, CheckCircle, XCircle, Loader2, RefreshCw } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    provider: ProviderConfig;
    apiKey: string;
    onApiKeyChange: (providerId: string, key: string) => void;
    onModelChange: (providerId: string, model: string) => void;
    onToggle?: (providerId: string) => void;
  }

  let { provider, apiKey, onApiKeyChange, onModelChange, onToggle }: Props = $props();

  let showKey = $state(false);
  let testing = $state(false);
  let testResult: { ok: boolean; message: string } | null = $state(null);
  let balance: { is_available: boolean; balance_infos: { currency: string; total_balance: string; granted_balance: string; topped_up_balance: string }[] } | null = $state(null);
  let models: ModelInfo[] = $state([]);
  let loadingModels = $state(false);
  let selectedModel = $state(provider.defaultModel);

  $effect(() => {
    getSetting(`default_model_${provider.id}`).then((saved) => {
      if (saved) selectedModel = saved;
    });
  });

  async function loadModels() {
    if (provider.needsAuth && !apiKey) {
      models = getDefaultModels(provider.id);
      return;
    }
    loadingModels = true;
    try {
      models = await getModelsCached(provider.id, apiKey || undefined);
    } catch {
      models = getDefaultModels(provider.id);
    } finally {
      loadingModels = false;
    }
  }

  async function handleRefresh() {
    if (provider.needsAuth && !apiKey) return;
    loadingModels = true;
    try {
      models = await refreshModels(provider.id, apiKey || undefined);
      addToast(`Models refreshed for ${provider.name}`, 'info');
    } catch {
      addToast(`Failed to refresh models for ${provider.name}`, 'error');
    } finally {
      loadingModels = false;
    }
  }

  async function handleTest() {
    testing = true;
    testResult = null;
    try {
      testResult = await testConnection(provider.id, apiKey);
      if (testResult.ok) {
        loadBalance();
      }
    } catch (e) {
      testResult = { ok: false, message: String(e) };
    } finally {
      testing = false;
    }
  }

  async function loadBalance() {
    if (!apiKey || !provider.balanceEndpoint) return;
    try {
      balance = await invoke('get_balance', { provider: provider.id });
    } catch {
      balance = null;
    }
  }

  $effect(() => {
    loadModels();
  });

  $effect(() => {
    if (apiKey && provider.balanceEndpoint) {
      loadBalance();
    }
  });
</script>

<div class="provider-selector">
  <div class="provider-header">
    <span class="provider-name">{provider.name}</span>
    <button class="provider-toggle" class:provider-toggle--on={provider.enabled} role="switch" aria-checked={provider.enabled} aria-label="Toggle {provider.name}"
      onclick={() => onToggle?.(provider.id)}>
    </button>
  </div>

  <div class="provider-fields">
    <div class="field">
      <label class="field-label" for="apikey-{provider.id}">{$LL.providers.apiKey()}</label>
      <div class="field-input-wrap">
        <input
          id="apikey-{provider.id}"
          type={showKey ? 'text' : 'password'}
          class="field-input"
          value={apiKey}
          oninput={(e) => onApiKeyChange(provider.id, e.currentTarget.value)}
          placeholder={$LL.providers.apiKeyPlaceholder() as string}
        />
        <button class="field-input-btn" onclick={() => (showKey = !showKey)} aria-label={$LL.aria.toggleKeyVisibility()}>
          {#if showKey}
            <EyeOff size={14} strokeWidth={1.5} />
          {:else}
            <Eye size={14} strokeWidth={1.5} />
          {/if}
        </button>
      </div>
    </div>

    <div class="field">
      <label class="field-label" for="model-{provider.id}">{$LL.providers.defaultModel()}</label>
      <div class="field-input-wrap">
        <select
          id="model-{provider.id}"
          class="field-input"
          value={selectedModel}
          onchange={(e) => {
            selectedModel = e.currentTarget.value;
            onModelChange(provider.id, e.currentTarget.value);
          }}
          disabled={loadingModels}
        >
          {#each models as m}
            <option value={m.id}>{m.name}</option>
          {/each}
        </select>
        <button class="field-input-btn" onclick={handleRefresh} disabled={!apiKey} aria-label={$LL.aria.refreshModels()}>
          {#if loadingModels}
            <Loader2 size={14} strokeWidth={1.5} class="spin" />
          {:else}
            <RefreshCw size={14} strokeWidth={1.5} />
          {/if}
        </button>
      </div>
    </div>

    <div class="field field--inline">
      <button class="test-btn" onclick={handleTest} disabled={testing || !apiKey}>
        {#if testing}
          <Loader2 size={14} strokeWidth={1.5} class="spin" />
        {:else}
          {$LL.test()}
        {/if}
      </button>

      {#if testResult}
        <span class="test-result" class:test-result--ok={testResult.ok} class:test-result--fail={!testResult.ok}>
          {#if testResult.ok}
            <CheckCircle size={13} strokeWidth={1.5} />
          {:else}
            <XCircle size={13} strokeWidth={1.5} />
          {/if}
          {testResult.message}
        </span>
      {/if}
    </div>

    {#if balance}
      {@const usd = balance.balance_infos.find(b => b.currency === 'USD')}
      {#if usd}
        <div class="balance" class:balance--ok={balance.is_available}>
          <span class="balance-dot"></span>
          ${usd.total_balance} USD
          {#if balance.is_available}{$LL.balanceAvailable()}{:else}{$LL.balanceUnavailable()}{/if}
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .provider-selector {
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 14px;
    background: var(--surface);
  }

  .provider-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .provider-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
  }

  .provider-toggle {
    width: 36px;
    height: 20px;
    border-radius: 10px;
    border: none;
    background: var(--surface-hover);
    cursor: pointer;
    position: relative;
  }

  .provider-toggle::after {
    content: '';
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: var(--text-dim);
    transition: transform 0.15s;
  }

  .provider-toggle--on {
    background: var(--primary);
  }

  .provider-toggle--on::after {
    transform: translateX(16px);
    background: white;
  }

  .provider-fields {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .field--inline {
    flex-direction: row;
    align-items: center;
    gap: 10px;
  }

  .field-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .field-input-wrap {
    display: flex;
    align-items: center;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 7px;
    overflow: hidden;
  }

  .field-input-wrap:focus-within {
    border-color: var(--primary);
  }

  .field-input {
    flex: 1;
    padding: 7px 10px;
    border: none;
    background: transparent;
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    outline: none;
    min-width: 0;
  }

  .field-input::placeholder {
    color: var(--text-dim);
  }

  select.field-input {
    cursor: pointer;
    appearance: none;
  }

  .field-input-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 7px 8px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
  }

  .field-input-btn:hover {
    color: var(--text);
  }

  .field-input-btn:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .test-btn {
    padding: 6px 14px;
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

  .test-btn:hover {
    background: var(--surface-hover);
  }

  .test-btn:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .test-result {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--text-dim);
    max-width: 240px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .test-result--ok {
    color: var(--success);
  }

  .test-result--fail {
    color: var(--error);
  }

  .balance {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    font-size: 12px;
    color: var(--text-dim);
  }

  .balance--ok {
    color: var(--text);
  }

  .balance-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--success);
    flex-shrink: 0;
  }

  .balance:not(.balance--ok) .balance-dot {
    background: var(--error);
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }
</style>
