export interface ProviderConfig {
  id: string;
  name: string;
  apiEndpoint: string;
  modelsEndpoint: string;
  balanceEndpoint?: string;
  defaultModel: string;
  enabled: boolean;
  needsAuth: boolean;
}

export const PROVIDERS: Record<string, ProviderConfig> = {
  deepseek: {
    id: 'deepseek',
    name: 'DeepSeek',
    apiEndpoint: 'https://api.deepseek.com/v1/chat/completions',
    modelsEndpoint: 'https://api.deepseek.com/v1/models',
    balanceEndpoint: 'https://api.deepseek.com/user/balance',
    defaultModel: 'deepseek-v4-pro',
    enabled: true,
    needsAuth: true,
  },
  opencode_go: {
    id: 'opencode_go',
    name: 'OpenCode Go',
    apiEndpoint: 'https://opencode.ai/zen/go/v1/chat/completions',
    modelsEndpoint: 'https://opencode.ai/zen/go/v1/models',
    defaultModel: 'kimi-k2.7-code',
    enabled: true,
    needsAuth: false,
  },
  opencode_zen: {
    id: 'opencode_zen',
    name: 'OpenCode Zen',
    apiEndpoint: 'https://opencode.ai/zen/v1/chat/completions',
    modelsEndpoint: 'https://opencode.ai/zen/v1/models',
    defaultModel: 'claude-sonnet-4-6',
    enabled: true,
    needsAuth: false,
  },
};

export function getProvider(id: string): ProviderConfig | undefined {
  return PROVIDERS[id];
}

export function getEnabledProviders(): ProviderConfig[] {
  return Object.values(PROVIDERS).filter((p) => p.enabled);
}

export function setProviderEnabled(id: string, enabled: boolean): void {
  const provider = PROVIDERS[id];
  if (provider) {
    provider.enabled = enabled;
  }
}
