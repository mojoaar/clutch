import { invoke } from '@tauri-apps/api/core';

export interface ModelInfo {
  id: string;
  name: string;
  provider: string;
  category: string | null;
  context_length: number | null;
}

let modelCache: Map<string, ModelInfo[]> = new Map();
let lastFetch: Map<string, number> = new Map();

export async function getModels(
  provider: string,
  apiKey?: string
): Promise<ModelInfo[]> {
  return invoke<ModelInfo[]>('get_models', { provider, apiKey });
}

export async function getCachedModels(provider: string): Promise<ModelInfo[]> {
  return invoke<ModelInfo[]>('get_cached_models', { provider });
}

export async function refreshModels(
  provider: string,
  apiKey?: string
): Promise<ModelInfo[]> {
  return invoke<ModelInfo[]>('refresh_models', { provider, apiKey });
}

export async function getModelsCached(
  provider: string,
  apiKey?: string
): Promise<ModelInfo[]> {
  const cached = modelCache.get(provider);
  const fetched = lastFetch.get(provider) || 0;
  const now = Date.now();

  if (cached && now - fetched < 24 * 60 * 60 * 1000) {
    return cached;
  }

  const models = await getModels(provider, apiKey);
  modelCache.set(provider, models);
  lastFetch.set(provider, now);
  return models;
}

export function getDefaultModels(provider: string): ModelInfo[] {
  const defaults: Record<string, ModelInfo[]> = {
    deepseek: [
      { id: 'deepseek-v4-pro', name: 'DeepSeek V4 Pro', provider: 'deepseek', category: 'DeepSeek', context_length: 128000 },
      { id: 'deepseek-v4-flash', name: 'DeepSeek V4 Flash', provider: 'deepseek', category: 'DeepSeek', context_length: 128000 },
    ],
    opencode_go: [
      { id: 'minimax-m3', name: 'MiniMax M3', provider: 'opencode_go', category: 'MiniMax', context_length: null },
      { id: 'minimax-m2.7', name: 'MiniMax M2.7', provider: 'opencode_go', category: 'MiniMax', context_length: null },
      { id: 'minimax-m2.5', name: 'MiniMax M2.5', provider: 'opencode_go', category: 'MiniMax', context_length: null },
      { id: 'kimi-k2.7-code', name: 'Kimi K2.7 Code', provider: 'opencode_go', category: 'Kimi', context_length: null },
      { id: 'kimi-k2.6', name: 'Kimi K2.6', provider: 'opencode_go', category: 'Kimi', context_length: null },
      { id: 'kimi-k2.5', name: 'Kimi K2.5', provider: 'opencode_go', category: 'Kimi', context_length: null },
      { id: 'glm-5.2', name: 'GLM 5.2', provider: 'opencode_go', category: 'GLM', context_length: null },
      { id: 'glm-5.1', name: 'GLM 5.1', provider: 'opencode_go', category: 'GLM', context_length: null },
      { id: 'glm-5', name: 'GLM 5', provider: 'opencode_go', category: 'GLM', context_length: null },
      { id: 'deepseek-v4-pro', name: 'DeepSeek V4 Pro', provider: 'opencode_go', category: 'DeepSeek', context_length: null },
      { id: 'deepseek-v4-flash', name: 'DeepSeek V4 Flash', provider: 'opencode_go', category: 'DeepSeek', context_length: null },
      { id: 'qwen3.7-max', name: 'Qwen 3.7 Max', provider: 'opencode_go', category: 'Qwen', context_length: null },
      { id: 'qwen3.7-plus', name: 'Qwen 3.7 Plus', provider: 'opencode_go', category: 'Qwen', context_length: null },
      { id: 'qwen3.6-plus', name: 'Qwen 3.6 Plus', provider: 'opencode_go', category: 'Qwen', context_length: null },
      { id: 'qwen3.5-plus', name: 'Qwen 3.5 Plus', provider: 'opencode_go', category: 'Qwen', context_length: null },
      { id: 'mimo-v2-pro', name: 'Mimo V2 Pro', provider: 'opencode_go', category: 'Mimo', context_length: null },
      { id: 'mimo-v2-omni', name: 'Mimo V2 Omni', provider: 'opencode_go', category: 'Mimo', context_length: null },
      { id: 'mimo-v2.5-pro', name: 'Mimo V2.5 Pro', provider: 'opencode_go', category: 'Mimo', context_length: null },
      { id: 'mimo-v2.5', name: 'Mimo V2.5', provider: 'opencode_go', category: 'Mimo', context_length: null },
      { id: 'hy3-preview', name: 'HY3 Preview', provider: 'opencode_go', category: 'Other', context_length: null },
    ],
    opencode_zen: [
      { id: 'claude-fable-5', name: 'Claude Fable 5', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-opus-4-8', name: 'Claude Opus 4.8', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-opus-4-7', name: 'Claude Opus 4.7', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-opus-4-6', name: 'Claude Opus 4.6', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-opus-4-5', name: 'Claude Opus 4.5', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-opus-4-1', name: 'Claude Opus 4.1', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-sonnet-4-6', name: 'Claude Sonnet 4.6', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-sonnet-4-5', name: 'Claude Sonnet 4.5', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-sonnet-4', name: 'Claude Sonnet 4', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'claude-haiku-4-5', name: 'Claude Haiku 4.5', provider: 'opencode_zen', category: 'Claude', context_length: null },
      { id: 'gemini-3.5-flash', name: 'Gemini 3.5 Flash', provider: 'opencode_zen', category: 'Gemini', context_length: null },
      { id: 'gemini-3.1-pro', name: 'Gemini 3.1 Pro', provider: 'opencode_zen', category: 'Gemini', context_length: null },
      { id: 'gemini-3-flash', name: 'Gemini 3 Flash', provider: 'opencode_zen', category: 'Gemini', context_length: null },
      { id: 'gpt-5.5', name: 'GPT 5.5', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.5-pro', name: 'GPT 5.5 Pro', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.4', name: 'GPT 5.4', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.4-pro', name: 'GPT 5.4 Pro', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.4-mini', name: 'GPT 5.4 Mini', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.4-nano', name: 'GPT 5.4 Nano', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.3-codex-spark', name: 'GPT 5.3 Codex Spark', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.3-codex', name: 'GPT 5.3 Codex', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.2', name: 'GPT 5.2', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.2-codex', name: 'GPT 5.2 Codex', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.1', name: 'GPT 5.1', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.1-codex-max', name: 'GPT 5.1 Codex Max', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.1-codex', name: 'GPT 5.1 Codex', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5.1-codex-mini', name: 'GPT 5.1 Codex Mini', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5', name: 'GPT 5', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5-codex', name: 'GPT 5 Codex', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'gpt-5-nano', name: 'GPT 5 Nano', provider: 'opencode_zen', category: 'GPT', context_length: null },
      { id: 'grok-build-0.1', name: 'Grok Build 0.1', provider: 'opencode_zen', category: 'Grok', context_length: null },
      { id: 'deepseek-v4-pro', name: 'DeepSeek V4 Pro', provider: 'opencode_zen', category: 'DeepSeek', context_length: null },
      { id: 'deepseek-v4-flash', name: 'DeepSeek V4 Flash', provider: 'opencode_zen', category: 'DeepSeek', context_length: null },
      { id: 'glm-5.2', name: 'GLM 5.2', provider: 'opencode_zen', category: 'GLM', context_length: null },
      { id: 'glm-5.1', name: 'GLM 5.1', provider: 'opencode_zen', category: 'GLM', context_length: null },
      { id: 'glm-5', name: 'GLM 5', provider: 'opencode_zen', category: 'GLM', context_length: null },
      { id: 'minimax-m2.7', name: 'MiniMax M2.7', provider: 'opencode_zen', category: 'MiniMax', context_length: null },
      { id: 'minimax-m2.5', name: 'MiniMax M2.5', provider: 'opencode_zen', category: 'MiniMax', context_length: null },
      { id: 'kimi-k2.6', name: 'Kimi K2.6', provider: 'opencode_zen', category: 'Kimi', context_length: null },
      { id: 'kimi-k2.5', name: 'Kimi K2.5', provider: 'opencode_zen', category: 'Kimi', context_length: null },
      { id: 'qwen3.6-plus', name: 'Qwen 3.6 Plus', provider: 'opencode_zen', category: 'Qwen', context_length: null },
      { id: 'qwen3.5-plus', name: 'Qwen 3.5 Plus', provider: 'opencode_zen', category: 'Qwen', context_length: null },
      { id: 'big-pickle', name: 'Big Pickle', provider: 'opencode_zen', category: 'Other', context_length: null },
      { id: 'deepseek-v4-flash-free', name: 'DeepSeek V4 Flash Free', provider: 'opencode_zen', category: 'Free', context_length: null },
      { id: 'mimo-v2.5-free', name: 'Mimo V2.5 Free', provider: 'opencode_zen', category: 'Free', context_length: null },
      { id: 'qwen3.6-plus-free', name: 'Qwen 3.6 Plus Free', provider: 'opencode_zen', category: 'Free', context_length: null },
      { id: 'minimax-m3-free', name: 'MiniMax M3 Free', provider: 'opencode_zen', category: 'Free', context_length: null },
      { id: 'nemotron-3-ultra-free', name: 'Nemotron 3 Ultra Free', provider: 'opencode_zen', category: 'Free', context_length: null },
      { id: 'north-mini-code-free', name: 'North Mini Code Free', provider: 'opencode_zen', category: 'Free', context_length: null },
    ],
  };

  return defaults[provider] || [];
}

export function groupModelsByCategory(models: ModelInfo[]): Map<string, ModelInfo[]> {
  const groups = new Map<string, ModelInfo[]>();
  for (const model of models) {
    const cat = model.category || 'Other';
    if (!groups.has(cat)) {
      groups.set(cat, []);
    }
    groups.get(cat)!.push(model);
  }
  return groups;
}

export function clearCache(provider?: string): void {
  if (provider) {
    modelCache.delete(provider);
    lastFetch.delete(provider);
  } else {
    modelCache.clear();
    lastFetch.clear();
  }
}
