import { writable } from 'svelte/store';
import { themes, applyColors, getEffectiveTheme, type ThemeName, type ThemeMode } from '$lib/themes';

const STORAGE_KEY_THEME = 'clutch:theme:name';
const STORAGE_KEY_MODE = 'clutch:theme:mode';

function loadThemeName(): ThemeName {
	if (typeof window === 'undefined') return 'clutch';
	const stored = localStorage.getItem(STORAGE_KEY_THEME);
	if (stored && stored in themes) return stored as ThemeName;
	return 'clutch';
}

function loadThemeMode(): ThemeMode {
	if (typeof window === 'undefined') return 'system';
	const stored = localStorage.getItem(STORAGE_KEY_MODE);
	if (stored === 'light' || stored === 'dark' || stored === 'system') return stored;
	return 'system';
}

export const themeName = writable<ThemeName>(loadThemeName());
export const themeMode = writable<ThemeMode>(loadThemeMode());

function applyCurrentTheme(name: ThemeName, mode: ThemeMode) {
	const variant = getEffectiveTheme(name, mode);
	applyColors(variant.colors);
}

if (typeof window !== 'undefined') {
	applyCurrentTheme(loadThemeName(), loadThemeMode());

	themeName.subscribe((name) => {
		localStorage.setItem(STORAGE_KEY_THEME, name);
		let mode: ThemeMode = 'system';
		themeMode.subscribe((m) => (mode = m))();
		applyCurrentTheme(name, mode);
	});

	themeMode.subscribe((mode) => {
		localStorage.setItem(STORAGE_KEY_MODE, mode);
		let name: ThemeName = 'clutch';
		themeName.subscribe((n) => (name = n))();
		applyCurrentTheme(name, mode);
	});

	window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
		let name: ThemeName = 'clutch';
		let mode: ThemeMode = 'system';
		themeName.subscribe((n) => (name = n))();
		themeMode.subscribe((m) => (mode = m))();
		if (mode === 'system') {
			applyCurrentTheme(name, mode);
		}
	});
}
