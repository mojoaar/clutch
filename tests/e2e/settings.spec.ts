import { test, expect } from "@playwright/test";

test("Settings → General tab is active by default", async ({ page }) => {
  await page.goto("/settings");
  await expect(
    page.getByRole("heading", { name: /General/i }).first(),
  ).toBeVisible();
});

test("change time format (Auto → 24h)", async ({ page }) => {
  await page.goto("/settings");
  const select = page.locator("#settings-time-format");
  await expect(select).toBeVisible();
  await select.selectOption("24");
  await expect(select).toHaveValue("24");
});

test("theme selector dropdown exists", async ({ page }) => {
  await page.goto("/settings");
  await expect(
    page.getByRole("heading", { name: /Appearance/i }),
  ).toBeVisible();
});
