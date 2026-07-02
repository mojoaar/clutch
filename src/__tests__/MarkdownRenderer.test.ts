// @vitest-environment jsdom
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';
import MarkdownRenderer from '$lib/components/MarkdownRenderer.svelte';

vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn().mockResolvedValue(null),
}));

describe('MarkdownRenderer', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		Object.defineProperty(navigator, 'clipboard', {
			value: { writeText: vi.fn().mockResolvedValue(undefined) },
			writable: true,
			configurable: true,
		});
	});

	afterEach(() => {
		cleanup();
	});

	it('renders bold markdown as <strong>', () => {
		const { container } = render(MarkdownRenderer, { props: { content: '**bold text**' } });
		const strong = container.querySelector('strong');
		expect(strong).toBeTruthy();
		expect(strong!.textContent).toBe('bold text');
	});

	it('renders code blocks with syntax highlighting class', () => {
		const { container } = render(MarkdownRenderer, {
			props: { content: '```javascript\nconst x = 1;\n```' },
		});
		const codeBlock = container.querySelector('.code-block-wrapper');
		expect(codeBlock).toBeTruthy();
		const code = container.querySelector('code.hljs');
		expect(code).toBeTruthy();
	});

	it('strips <script> tags from content (XSS prevention)', () => {
		const { container } = render(MarkdownRenderer, {
			props: { content: '<script>alert("xss")</script>' },
		});
		expect(container.innerHTML).not.toContain('<script>');
		expect(container.innerHTML).not.toContain('alert');
	});

	it('renders empty content as nothing', () => {
		const { container } = render(MarkdownRenderer, {
			props: { content: '' },
		});
		const markdown = container.querySelector('.markdown');
		expect(markdown).toBeNull();
	});

	it('renders tables correctly', () => {
		const { container } = render(MarkdownRenderer, {
			props: { content: '| A | B |\n|---|---|\n| 1 | 2 |' },
		});
		const table = container.querySelector('table');
		expect(table).toBeTruthy();
		const cells = table!.querySelectorAll('td');
		expect(cells.length).toBe(2);
		expect(cells[0].textContent).toBe('1');
		expect(cells[1].textContent).toBe('2');
	});

	it('copy code button calls navigator.clipboard.writeText', async () => {
		const { container } = render(MarkdownRenderer, {
			props: { content: '```typescript\ntest code\n```' },
		});

		const copyButton = container.querySelector('.code-block-copy') as HTMLElement;
		expect(copyButton).toBeTruthy();

		copyButton.click();
		await vi.waitFor(() => {
			expect(navigator.clipboard.writeText).toHaveBeenCalledWith(expect.stringContaining('test code'));
		});
	});
});
