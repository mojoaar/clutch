import { test, expect } from '@playwright/test';

test('Settings → Export tab shows format options', async ({ page }) => {
	await page.goto('/settings?tab=export');
	await expect(page.getByRole('heading', { name: /Default Export/i })).toBeVisible();
});

test('toggle include metadata checkbox', async ({ page }) => {
	await page.goto('/settings?tab=export');
	const checkbox = page.getByRole('checkbox').first();
	if (await checkbox.isVisible()) {
		await checkbox.click();
	}
});
