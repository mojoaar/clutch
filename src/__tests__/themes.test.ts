import { describe, it, expect } from 'vitest';
import { themes, getEffectiveTheme, type ThemeName, type ThemeMode } from '$lib/themes';

describe('themes', () => {
	it('contains all expected theme names', () => {
		const names = Object.keys(themes);
		expect(names).toContain('clutch');
		expect(names).toContain('nord');
		expect(names).toContain('dracula');
		expect(names).toContain('cyberpunk');
		expect(names).toContain('catppuccin');
		expect(names).toContain('github');
		expect(names).toContain('tokyo-night');
		expect(names).toContain('monokai');
	});

	it('each theme has light and dark variants', () => {
		for (const [, theme] of Object.entries(themes)) {
			expect(theme.light).toBeDefined();
			expect(theme.dark).toBeDefined();
			expect(theme.light.name).toBe('light');
			expect(theme.dark.name).toBe('dark');
		}
	});

	it('each theme variant has all required color keys', () => {
		const requiredColors = [
			'--color-bg',
			'--color-surface',
			'--color-surface-hover',
			'--color-border',
			'--color-text',
			'--color-text-muted',
			'--color-text-dim',
			'--color-primary',
			'--color-primary-hover',
			'--color-accent',
			'--color-success',
			'--color-warning',
			'--color-error',
			'--color-code-bg',
			'--color-scrollbar',
			'--color-shadow',
		];

		for (const [, theme] of Object.entries(themes)) {
			for (const variant of [theme.light, theme.dark]) {
				for (const key of requiredColors) {
					expect(variant.colors[key as keyof typeof variant.colors]).toBeDefined();
				}
			}
		}
	});
});

describe('getEffectiveTheme', () => {
	it('returns light variant when mode is "light"', () => {
		const variant = getEffectiveTheme('nord', 'light');
		expect(variant.name).toBe('light');
		expect(variant.colors['--color-bg']).toBe('#eceff4');
	});

	it('returns dark variant when mode is "dark"', () => {
		const variant = getEffectiveTheme('nord', 'dark');
		expect(variant.name).toBe('dark');
		expect(variant.colors['--color-bg']).toBe('#2e3440');
	});

	it('returns light variant when mode is "system" and prefers-color-scheme is absent (SSR)', () => {
		// In Node/SSR, window.matchMedia is not available, so it falls back to light
		const variant = getEffectiveTheme('clutch', 'system');
		expect(variant.name).toBe('light');
	});

	it('returns the correct variant for all themes', () => {
		const themeNames: ThemeName[] = ['clutch', 'dracula', 'cyberpunk'];
		for (const name of themeNames) {
			expect(getEffectiveTheme(name, 'light').name).toBe('light');
			expect(getEffectiveTheme(name, 'dark').name).toBe('dark');
		}
	});

	it('has distinct light and dark colors for each theme', () => {
		for (const [, theme] of Object.entries(themes)) {
			expect(theme.light.colors).not.toEqual(theme.dark.colors);
		}
	});
});
