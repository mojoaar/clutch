import { test, expect } from '@playwright/test';

test('Settings → Providers tab is accessible', async ({ page }) => {
	await page.goto('/settings?tab=providers');
	await expect(page.getByRole('heading', { name: /API Providers/i })).toBeVisible();
});

test('API key input exists and is password type', async ({ page }) => {
	await page.goto('/settings?tab=providers');
	const input = page.locator('#apikey-deepseek');
	await expect(input).toBeVisible();
	await expect(input).toHaveAttribute('type', 'password');
});

test('test connection button exists', async ({ page }) => {
	await page.goto('/settings?tab=providers');
	await expect(page.getByRole('button', { name: 'Test' })).toBeVisible();
});
