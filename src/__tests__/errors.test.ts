import { describe, it, expect, vi } from 'vitest';
import {
	errorMessage,
	errorToastVariant,
	isRetryable,
	retryWithBackoff,
	type AppError,
} from '$lib/services/errors';

describe('errorMessage', () => {
	it('formats an API error', () => {
		const err: AppError = {
			kind: 'api',
			provider: 'DeepSeek',
			status: 401,
			message: 'Invalid API key',
		};
		expect(errorMessage(err)).toBe('DeepSeek API error (401): Invalid API key');
	});

	it('formats a network error', () => {
		const err: AppError = { kind: 'network', detail: 'Connection refused' };
		expect(errorMessage(err)).toBe('Network error: Connection refused');
	});

	it('formats a file system error', () => {
		const err: AppError = {
			kind: 'file_system',
			operation: 'read',
			path: '/tmp/test.txt',
			detail: 'Permission denied',
		};
		expect(errorMessage(err)).toBe(
			'File system error (read on /tmp/test.txt): Permission denied',
		);
	});

	it('formats a parse error', () => {
		const err: AppError = {
			kind: 'parse',
			source: 'JSON',
			detail: 'Unexpected token',
		};
		expect(errorMessage(err)).toBe('Parse error (JSON): Unexpected token');
	});

	it('formats a validation error', () => {
		const err: AppError = {
			kind: 'validation',
			field: 'email',
			detail: 'Invalid format',
		};
		expect(errorMessage(err)).toBe('Invalid email: Invalid format');
	});

	it('formats a timeout error', () => {
		const err: AppError = { kind: 'timeout', operation: 'API request' };
		expect(errorMessage(err)).toBe('API request timed out');
	});

	it('formats a stream interrupted error', () => {
		const err: AppError = {
			kind: 'stream_interrupted',
			sessionId: 'abc-123',
			partialContent: 'Hello',
		};
		expect(errorMessage(err)).toBe('Response stream interrupted for session abc-123');
	});
});

describe('errorToastVariant', () => {
	it('returns "error" for 5xx API errors', () => {
		const err: AppError = {
			kind: 'api',
			provider: 'DeepSeek',
			status: 500,
			message: 'Server error',
		};
		expect(errorToastVariant(err)).toBe('error');
	});

	it('returns "warning" for 4xx API errors', () => {
		const err: AppError = {
			kind: 'api',
			provider: 'DeepSeek',
			status: 404,
			message: 'Not found',
		};
		expect(errorToastVariant(err)).toBe('warning');
	});

	it('returns "warning" for network errors', () => {
		const err: AppError = { kind: 'network', detail: 'Timeout' };
		expect(errorToastVariant(err)).toBe('warning');
	});

	it('returns "error" for validation errors', () => {
		const err: AppError = {
			kind: 'validation',
			field: 'name',
			detail: 'Required',
		};
		expect(errorToastVariant(err)).toBe('error');
	});
});

describe('isRetryable', () => {
	it('returns true for network errors', () => {
		expect(isRetryable({ kind: 'network', detail: 'Offline' })).toBe(true);
	});

	it('returns true for timeout errors', () => {
		expect(
			isRetryable({ kind: 'timeout', operation: 'fetch' }),
		).toBe(true);
	});

	it('returns true for stream interrupted', () => {
		expect(
			isRetryable({
				kind: 'stream_interrupted',
				sessionId: 'x',
				partialContent: '',
			}),
		).toBe(true);
	});

	it('returns false for API errors', () => {
		expect(
			isRetryable({
				kind: 'api',
				provider: 'D',
				status: 500,
				message: 'Err',
			}),
		).toBe(false);
	});

	it('returns false for validation errors', () => {
		expect(
			isRetryable({
				kind: 'validation',
				field: 'x',
				detail: 'bad',
			}),
		).toBe(false);
	});
});

describe('retryWithBackoff', () => {
	it('returns the result on first success', async () => {
		const fn = vi.fn().mockResolvedValue('ok');
		const result = await retryWithBackoff(fn, () => false, 3, 10);
		expect(result).toBe('ok');
		expect(fn).toHaveBeenCalledTimes(1);
	});

	it('retries on retryable errors and succeeds', async () => {
		const fn = vi
			.fn()
			.mockRejectedValueOnce(new Error('fail'))
			.mockRejectedValueOnce(new Error('fail'))
			.mockResolvedValue('success');

		const result = await retryWithBackoff(
			fn,
			() => true,
			3,
			10,
		);
		expect(result).toBe('success');
		expect(fn).toHaveBeenCalledTimes(3);
	});

	it('throws after max retries', async () => {
		const fn = vi.fn().mockRejectedValue(new Error('persistent failure'));

		await expect(
			retryWithBackoff(fn, () => true, 2, 10),
		).rejects.toThrow('persistent failure');
		expect(fn).toHaveBeenCalledTimes(3); // initial + 2 retries
	});

	it('throws immediately on non-retryable errors', async () => {
		const fn = vi.fn().mockRejectedValue(new Error('bad input'));

		await expect(
			retryWithBackoff(fn, () => false, 3, 10),
		).rejects.toThrow('bad input');
		expect(fn).toHaveBeenCalledTimes(1);
	});
});
