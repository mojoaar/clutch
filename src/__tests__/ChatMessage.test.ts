// @vitest-environment jsdom
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';
import ChatMessage from '$lib/components/ChatMessage.svelte';
import { devMode } from '$lib/stores/dev';
import type { Message } from '$lib/stores/chat';

vi.mock('$lib/db', () => ({
	getUserProfile: vi.fn().mockResolvedValue(null),
}));

vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn().mockResolvedValue(null),
}));

function createMessage(overrides: Partial<Message> = {}): Message {
	return {
		id: 'msg-1',
		sessionId: 'session-1',
		role: 'user',
		content: 'Hello, world!',
		createdAt: '2025-01-01T00:00:00Z',
		...overrides,
	};
}

describe('ChatMessage', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		Object.defineProperty(navigator, 'clipboard', {
			value: { writeText: vi.fn().mockResolvedValue(undefined) },
			writable: true,
			configurable: true,
		});
		devMode.set(false);
	});

	afterEach(() => {
		devMode.set(false);
		cleanup();
	});

	it('renders user message with correct role label', () => {
		const msg = createMessage({ role: 'user', content: 'Hello!' });
		render(ChatMessage, { props: { message: msg } });
		expect(screen.getByText('Hello!')).toBeTruthy();
	});

	it('renders assistant message with provider label', () => {
		const msg = createMessage({
			role: 'assistant',
			content: 'Response',
			provider: 'deepseek',
			model: 'deepseek-v4-pro',
		});
		render(ChatMessage, { props: { message: msg } });
		expect(screen.getByText('Response')).toBeTruthy();
	});

	it('renders deleted message placeholder', () => {
		const msg = createMessage({ isDeleted: true });
		render(ChatMessage, { props: { message: msg } });
		expect(screen.getByText('This message was deleted.')).toBeTruthy();
	});

	it('copy button exists and calls navigator.clipboard.writeText', async () => {
		const msg = createMessage({ role: 'user', content: 'Copy me' });
		render(ChatMessage, { props: { message: msg } });

		const copyBtn = screen.getByRole('button', { name: 'Copy message' });
		expect(copyBtn).toBeTruthy();

		copyBtn.click();
		await vi.waitFor(() => {
			expect(navigator.clipboard.writeText).toHaveBeenCalledWith('Copy me');
		});
	});

	it('shows regenerate button for assistant messages', () => {
		const onRegenerate = vi.fn();
		const msg = createMessage({ role: 'assistant' });
		render(ChatMessage, { props: { message: msg, onRegenerate } });

		expect(screen.getByRole('button', { name: 'Regenerate response' })).toBeTruthy();
	});

	it('does not show regenerate button for user messages', () => {
		const onRegenerate = vi.fn();
		const msg = createMessage({ role: 'user' });
		render(ChatMessage, { props: { message: msg, onRegenerate } });

		const regenerateBtn = screen.queryByRole('button', { name: 'Regenerate response' });
		expect(regenerateBtn).toBeNull();
	});

	it('shows edit indicator when message.editedAt is set', () => {
		const msg = createMessage({ editedAt: '2025-01-02T00:00:00Z' });
		render(ChatMessage, { props: { message: msg } });

		expect(screen.getByText('(edited)')).toBeTruthy();
	});

	it('shows token count when dev mode is enabled and tokensUsed is set', () => {
		devMode.set(true);
		const msg = createMessage({ tokensUsed: 42 });
		render(ChatMessage, { props: { message: msg } });

		expect(screen.getByText(/42/)).toBeTruthy();
	});

	it('does not show token count when dev mode is off', () => {
		devMode.set(false);
		const msg = createMessage({ tokensUsed: 42 });
		render(ChatMessage, { props: { message: msg } });

		expect(screen.queryByText(/42/)).toBeNull();
	});
});
