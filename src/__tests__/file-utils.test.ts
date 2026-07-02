import { describe, it, expect } from 'vitest';
import { formatFileSize, getFileType, isAllowedFileType } from '$lib/utils/file-utils';

describe('formatFileSize', () => {
	it('returns "0 B" for zero bytes', () => {
		expect(formatFileSize(0)).toBe('0 B');
	});

	it('formats bytes', () => {
		expect(formatFileSize(500)).toBe('500 B');
	});

	it('formats kilobytes with one decimal', () => {
		expect(formatFileSize(1500)).toBe('1.5 KB');
	});

	it('formats megabytes', () => {
		expect(formatFileSize(1048576)).toBe('1.0 MB');
	});

	it('formats gigabytes', () => {
		expect(formatFileSize(1073741824)).toBe('1.0 GB');
	});

	it('caps at GB for very large sizes', () => {
		expect(formatFileSize(1099511627776)).toBe('1024.0 GB');
	});

	it('rounds correctly at boundary', () => {
		expect(formatFileSize(1024)).toBe('1.0 KB');
		expect(formatFileSize(1023)).toBe('1023 B');
	});
});

describe('getFileType', () => {
	it('detects TypeScript by extension', () => {
		const file = { name: 'app.ts', type: '' } as File;
		expect(getFileType(file)).toBe('typescript');
	});

	it('detects Svelte by extension', () => {
		const file = { name: 'Page.svelte', type: '' } as File;
		expect(getFileType(file)).toBe('svelte');
	});

	it('detects Python by extension', () => {
		const file = { name: 'main.py', type: '' } as File;
		expect(getFileType(file)).toBe('python');
	});

	it('detects Rust by extension', () => {
		const file = { name: 'main.rs', type: '' } as File;
		expect(getFileType(file)).toBe('rust');
	});

	it('detects Go by extension', () => {
		const file = { name: 'main.go', type: '' } as File;
		expect(getFileType(file)).toBe('go');
	});

	it('detects JavaScript by .js extension', () => {
		const file = { name: 'app.js', type: '' } as File;
		expect(getFileType(file)).toBe('javascript');
	});

	it('detects markdown by .md extension', () => {
		const file = { name: 'README.md', type: '' } as File;
		expect(getFileType(file)).toBe('markdown');
	});

	it('detects JSON', () => {
		const file = { name: 'package.json', type: '' } as File;
		expect(getFileType(file)).toBe('json');
	});

	it('falls back to "text" for unknown extensions', () => {
		const file = { name: 'unknown.xyz', type: '' } as File;
		expect(getFileType(file)).toBe('text');
	});

	it('normalizes uppercase extensions', () => {
		const file = { name: 'App.TS', type: '' } as File;
		expect(getFileType(file)).toBe('typescript');
	});
});

describe('isAllowedFileType', () => {
	it('allows image files', () => {
		const file = { name: 'photo.png', type: 'image/png' } as File;
		expect(isAllowedFileType(file)).toBe(true);
	});

	it('allows code files by extension', () => {
		const file = { name: 'app.ts', type: '' } as File;
		expect(isAllowedFileType(file)).toBe(true);
	});

	it('rejects disallowed file types', () => {
		const file = { name: 'movie.mp4', type: 'video/mp4' } as File;
		expect(isAllowedFileType(file)).toBe(false);
	});

	it('allows text/plain files', () => {
		const file = { name: 'notes.txt', type: 'text/plain' } as File;
		expect(isAllowedFileType(file)).toBe(true);
	});
});
