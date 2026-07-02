import { describe, it, expect, vi, beforeEach, beforeAll, afterEach } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn(),
	Channel: vi.fn(function (this: { onmessage: ((data: string) => void) | null }) {
		this.onmessage = null;
	}),
}));

vi.mock('$lib/stores/chat', () => ({
	chatStore: {
		subscribe: vi.fn(),
		addMessage: vi.fn(),
		setStreamingStatus: vi.fn(),
		setStreamingTokens: vi.fn(),
		appendToMessage: vi.fn(),
	},
}));

vi.mock('$lib/db', () => ({
	createMessage: vi.fn().mockResolvedValue(undefined),
}));

vi.mock('$lib/stores/toast', () => ({
	addToast: vi.fn(),
}));

vi.mock('$lib/services/network', () => ({
	network: {
		enqueue: vi.fn(),
		queueSize: {
			subscribe: vi.fn(() => () => {}),
		},
	},
	registerQueueProcessor: vi.fn(),
}));

vi.mock('$lib/i18n/i18n-svelte', () => {
	let value: any = {
		networkStatus: { messageQueued: () => 'Message queued for retry' },
		errors: { apiError: () => 'API error' },
	};
	const subs: Array<(v: any) => void> = [];
	const store = {
		subscribe(fn: (v: any) => void) {
			fn(value);
			subs.push(fn);
			return () => {};
		},
		set(v: any) {
			value = v;
			subs.forEach((f) => f(v));
		},
		update(fn: (v: any) => any) {
			store.set(fn(value));
		},
	};
	return { default: store };
});

import { invoke, Channel } from '@tauri-apps/api/core';
import { chatStore } from '$lib/stores/chat';
import { network, registerQueueProcessor } from '$lib/services/network';
import { streamChat, sendMessage } from '$lib/services/api';

function getLastChannel(): { onmessage: ((data: string) => void) | null } | null {
	const results = (Channel as any).mock.results;
	return results.length > 0 ? results[results.length - 1].value : null;
}

describe('sendMessage', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('enqueues instead of calling invoke when offline', async () => {
		vi.stubGlobal('navigator', { onLine: false });

		await sendMessage('session-1', 'Hello', 'deepseek', 'deepseek-chat', []);

		expect(network.enqueue).toHaveBeenCalledOnce();
		expect(network.enqueue).toHaveBeenCalledWith(
			'sendMessage',
			['session-1', 'Hello', 'deepseek', 'deepseek-chat', [], undefined, undefined],
			3,
		);
		expect(invoke).not.toHaveBeenCalled();
	});

	it('calls invoke with correct args when online', async () => {
		vi.stubGlobal('navigator', { onLine: true });
		(invoke as any).mockResolvedValue(undefined);

		await sendMessage('session-1', 'Hello', 'deepseek', 'deepseek-chat', []);

		expect(invoke).toHaveBeenCalledTimes(1);
		const [cmd, args] = (invoke as any).mock.calls[0];
		expect(cmd).toBe('stream_chat');
		expect(args.request.provider).toBe('deepseek');
		expect(args.request.model).toBe('deepseek-chat');
		expect(args.request.messages).toHaveLength(1);
		expect(args.request.messages[0].role).toBe('user');
		expect(args.request.messages[0].content).toBe('Hello');
	});

	it('includes contextContent as system message when provided', async () => {
		vi.stubGlobal('navigator', { onLine: true });
		(invoke as any).mockResolvedValue(undefined);

		await sendMessage('session-1', 'Hello', 'deepseek', 'deepseek-chat', [], undefined, 'fetched content');

		const [, args] = (invoke as any).mock.calls[0];
		expect(args.request.messages).toHaveLength(2);
		expect(args.request.messages[1].role).toBe('system');
		expect(args.request.messages[1].content).toContain('fetched content');
	});
});

describe('streamChat', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('sets streaming status and initializes assistant message', async () => {
		(invoke as any).mockReturnValue(new Promise(() => {}));

		const _promise = streamChat({
			provider: 'deepseek',
			model: 'deepseek-chat',
			messages: [{ role: 'user', content: 'Hi' }],
			sessionId: 'session-1',
		});

		expect(chatStore.setStreamingStatus).toHaveBeenCalledWith('streaming');
		expect(chatStore.setStreamingTokens).toHaveBeenCalledWith(0);
		expect(chatStore.addMessage).toHaveBeenCalledTimes(1);
		expect((chatStore.addMessage as any).mock.calls[0][0].role).toBe('assistant');
		expect((chatStore.addMessage as any).mock.calls[0][0].sessionId).toBe('session-1');
	});

	it('parses __TOKENS__:NN and sets streaming tokens', async () => {
		(invoke as any).mockReturnValue(new Promise(() => {}));

		const _promise = streamChat({
			provider: 'deepseek',
			model: 'deepseek-chat',
			messages: [{ role: 'user', content: 'Hi' }],
			sessionId: 'session-1',
		});

		const channel = getLastChannel();
		expect(channel).toBeDefined();
		channel!.onmessage!('Hello');
		channel!.onmessage!('__TOKENS__:42');

		expect(chatStore.setStreamingTokens).toHaveBeenCalledWith(42);
	});

	it('accumulates text tokens into message content', async () => {
		(invoke as any).mockReturnValue(new Promise(() => {}));

		const _promise = streamChat({
			provider: 'deepseek',
			model: 'deepseek-chat',
			messages: [{ role: 'user', content: 'Hi' }],
			sessionId: 'session-1',
		});

		const channel = getLastChannel();
		const assistantId = (chatStore.addMessage as any).mock.calls[0][0].id;

		channel!.onmessage!('Hello');
		channel!.onmessage!(' World');

		expect(chatStore.appendToMessage).toHaveBeenCalledTimes(2);
		expect(chatStore.appendToMessage).toHaveBeenNthCalledWith(1, assistantId, 'Hello');
		expect(chatStore.appendToMessage).toHaveBeenNthCalledWith(2, assistantId, ' World');
	});

	it('handles __STREAM_INTERRUPTED__ and sets interrupted status', async () => {
		(invoke as any).mockReturnValue(new Promise(() => {}));

		const _promise = streamChat({
			provider: 'deepseek',
			model: 'deepseek-chat',
			messages: [{ role: 'user', content: 'Hi' }],
			sessionId: 'session-1',
		});

		const channel = getLastChannel();
		channel!.onmessage!('some text');
		channel!.onmessage!('__STREAM_INTERRUPTED__');

		const calls = (chatStore.setStreamingStatus as any).mock.calls.map((c: any) => c[0]);
		expect(calls).toContain('interrupted');
	});

	it('prevents complete status overwrite after interruption (wasInterrupted flag)', async () => {
		(invoke as any).mockReturnValue(new Promise(() => {}));

		const _promise = streamChat({
			provider: 'deepseek',
			model: 'deepseek-chat',
			messages: [{ role: 'user', content: 'Hi' }],
			sessionId: 'session-1',
		});

		const channel = getLastChannel();
		channel!.onmessage!('__STREAM_INTERRUPTED__');
		channel!.onmessage!('__TOKENS__:99');

		const calls = (chatStore.setStreamingStatus as any).mock.calls.map((c: any) => c[0]);
		expect(calls).toContain('interrupted');
		expect(calls).not.toContain('complete');
	});

	it('appends error message content on invoke rejection', async () => {
		(invoke as any).mockRejectedValue(new Error('Network failure'));

		await streamChat({
			provider: 'deepseek',
			model: 'deepseek-chat',
			messages: [{ role: 'user', content: 'Hi' }],
			sessionId: 'session-1',
		});

		const appendCalls = (chatStore.appendToMessage as any).mock.calls.map((c: any) => c[1]);
		const errorCall = appendCalls.find((c: string) => c.includes('Network failure'));
		expect(errorCall).toBeDefined();
	});

	it('does not enqueue retry for non-retryable errors', async () => {
		(invoke as any).mockRejectedValue(new Error('Invalid API key'));

		await streamChat({
			provider: 'deepseek',
			model: 'deepseek-chat',
			messages: [{ role: 'user', content: 'Hi' }],
			sessionId: 'session-1',
		});

		expect(network.enqueue).not.toHaveBeenCalled();
	});
});

const _rpWasCalledAtImport = (registerQueueProcessor as any).mock.calls.length > 0;

describe('api module registration', () => {
	it('registers queue processor for retryable operations at import time', () => {
		expect(_rpWasCalledAtImport).toBe(true);
	});
});
