import { describe, it, expect, afterEach } from 'vitest';
import {
	getProvider,
	getEnabledProviders,
	setProviderEnabled,
	PROVIDERS,
	type ProviderConfig,
} from '$lib/services/providers';

describe('getProvider', () => {
	it('returns the deepseek provider config', () => {
		const provider = getProvider('deepseek');
		expect(provider).toBeDefined();
		expect(provider!.id).toBe('deepseek');
		expect(provider!.name).toBe('DeepSeek');
		expect(provider!.needsAuth).toBe(true);
	});

	it('returns the opencode_go provider config', () => {
		const provider = getProvider('opencode_go');
		expect(provider).toBeDefined();
		expect(provider!.name).toBe('OpenCode Go');
	});

	it('returns the opencode_zen provider config', () => {
		const provider = getProvider('opencode_zen');
		expect(provider).toBeDefined();
		expect(provider!.name).toBe('OpenCode Zen');
	});

	it('returns undefined for unknown provider', () => {
		expect(getProvider('nonexistent')).toBeUndefined();
	});
});

describe('getEnabledProviders', () => {
	afterEach(() => {
		// Reset all providers to enabled after each test
		for (const id of Object.keys(PROVIDERS)) {
			setProviderEnabled(id, true);
		}
	});

	it('returns all providers by default', () => {
		const enabled = getEnabledProviders();
		expect(enabled.length).toBe(3);
	});

	it('excludes disabled providers', () => {
		setProviderEnabled('deepseek', false);
		const enabled = getEnabledProviders();
		expect(enabled.length).toBe(2);
		expect(enabled.find((p) => p.id === 'deepseek')).toBeUndefined();
	});

	it('can disable and re-enable a provider', () => {
		setProviderEnabled('opencode_go', false);
		expect(getEnabledProviders().length).toBe(2);
		setProviderEnabled('opencode_go', true);
		expect(getEnabledProviders().length).toBe(3);
	});

	it('ignores setProviderEnabled for unknown providers', () => {
		setProviderEnabled('unknown', false);
		expect(getEnabledProviders().length).toBe(3);
	});
});

describe('PROVIDERS constant', () => {
	it('has valid API endpoints for all providers', () => {
		for (const [, config] of Object.entries(PROVIDERS)) {
			expect(config.apiEndpoint).toMatch(/^https:\/\//);
			expect(config.modelsEndpoint).toMatch(/^https:\/\//);
		}
	});

	it('has defaultModel for all providers', () => {
		for (const [, config] of Object.entries(PROVIDERS)) {
			expect(config.defaultModel).toBeTruthy();
		}
	});
});
