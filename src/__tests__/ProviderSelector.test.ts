// @vitest-environment jsdom
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import ProviderSelector from '$lib/components/ProviderSelector.svelte';
import type { ProviderConfig } from '$lib/services/providers';

vi.mock('$lib/services/models', () => ({
	getModelsCached: vi.fn().mockResolvedValue([
		{ id: 'model-1', name: 'Model One' },
		{ id: 'model-2', name: 'Model Two' },
	]),
	refreshModels: vi.fn().mockResolvedValue([
		{ id: 'model-1', name: 'Model One' },
	]),
	getDefaultModels: vi.fn().mockReturnValue([
		{ id: 'default-model', name: 'Default Model' },
	]),
}));

vi.mock('$lib/services/settings', () => ({
	testConnection: vi.fn().mockResolvedValue({ ok: true, message: 'Connected' }),
	getSetting: vi.fn().mockResolvedValue(null),
}));

vi.mock('$lib/stores/toast', () => ({
	addToast: vi.fn(),
}));

vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn().mockResolvedValue(null),
}));

function createProvider(overrides: Partial<ProviderConfig> = {}): ProviderConfig {
	return {
		id: 'deepseek',
		name: 'DeepSeek',
		apiEndpoint: 'https://api.deepseek.com/v1/chat/completions',
		modelsEndpoint: 'https://api.deepseek.com/v1/models',
		defaultModel: 'deepseek-v4-pro',
		enabled: true,
		needsAuth: true,
		...overrides,
	};
}

describe('ProviderSelector', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	afterEach(() => {
		cleanup();
	});

	it('renders provider name from props', () => {
		const provider = createProvider({ name: 'DeepSeek' });
		render(ProviderSelector, {
			props: {
				provider,
				apiKey: '',
				onApiKeyChange: vi.fn(),
				onModelChange: vi.fn(),
			},
		});
		expect(screen.getByText('DeepSeek')).toBeTruthy();
	});

	it('shows enabled state via toggle aria-checked', () => {
		const provider = createProvider({ enabled: true });
		render(ProviderSelector, {
			props: {
				provider,
				apiKey: '',
				onApiKeyChange: vi.fn(),
				onModelChange: vi.fn(),
			},
		});
		const toggle = screen.getByRole('switch');
		expect(toggle.getAttribute('aria-checked')).toBe('true');
	});

	it('shows disabled state via toggle aria-checked', () => {
		const provider = createProvider({ enabled: false });
		render(ProviderSelector, {
			props: {
				provider,
				apiKey: '',
				onApiKeyChange: vi.fn(),
				onModelChange: vi.fn(),
			},
		});
		const toggle = screen.getByRole('switch');
		expect(toggle.getAttribute('aria-checked')).toBe('false');
	});

	it('toggle button fires onToggle callback', async () => {
		const onToggle = vi.fn();
		const provider = createProvider();
		render(ProviderSelector, {
			props: {
				provider,
				apiKey: '',
				onApiKeyChange: vi.fn(),
				onModelChange: vi.fn(),
				onToggle,
			},
		});
		const toggle = screen.getByRole('switch');
		await fireEvent.click(toggle);
		expect(onToggle).toHaveBeenCalledWith('deepseek');
	});

	it('refresh button is disabled when no API key provided', () => {
		const provider = createProvider({ needsAuth: true });
		render(ProviderSelector, {
			props: {
				provider,
				apiKey: '',
				onApiKeyChange: vi.fn(),
				onModelChange: vi.fn(),
			},
		});
		const refreshBtn = screen.getByRole('button', { name: 'Refresh models' });
		expect(refreshBtn).toBeTruthy();
		expect((refreshBtn as HTMLButtonElement).disabled).toBe(true);
	});

	it('API key input is password type by default', () => {
		const provider = createProvider();
		render(ProviderSelector, {
			props: {
				provider,
				apiKey: 'sk-test-key',
				onApiKeyChange: vi.fn(),
				onModelChange: vi.fn(),
			},
		});
		const input = screen.getByPlaceholderText('sk-...') as HTMLInputElement;
		expect(input.type).toBe('password');
	});

	it('test connection button is disabled when no API key', () => {
		const provider = createProvider({ needsAuth: true });
		render(ProviderSelector, {
			props: {
				provider,
				apiKey: '',
				onApiKeyChange: vi.fn(),
				onModelChange: vi.fn(),
			},
		});
		const testBtn = screen.getByRole('button', { name: 'Test' });
		expect((testBtn as HTMLButtonElement).disabled).toBe(true);
	});
});
