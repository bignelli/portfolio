import { test, expect } from "@playwright/test";

test.describe("Theme Switching (Dark / Light Mode)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("should start in dark mode by default", async ({ page }) => {
    const app = page.locator(".portfolio-app");
    await expect(app).toHaveClass(/theme-dark/);
    await expect(app).toHaveAttribute("data-theme", "dark");

    const modeBtn = page.locator('.control-btn:has-text("MODE:")');
    await expect(modeBtn.locator(".control-val")).toHaveText("LIGHT");
  });

  test("should toggle from dark to light mode on click", async ({ page }) => {
    const app = page.locator(".portfolio-app");
    const modeBtn = page.locator('.control-btn:has-text("MODE:")');

    await modeBtn.click();

    await expect(app).toHaveClass(/theme-light/);
    await expect(app).not.toHaveClass(/theme-dark/);
    await expect(app).toHaveAttribute("data-theme", "light");
    await expect(modeBtn.locator(".control-val")).toHaveText("DARK");
  });

  test("should toggle back to dark mode on second click", async ({ page }) => {
    const app = page.locator(".portfolio-app");
    const modeBtn = page.locator('.control-btn:has-text("MODE:")');

    // First click: light
    await modeBtn.click();
    await expect(app).toHaveAttribute("data-theme", "light");

    // Second click: dark
    await modeBtn.click();
    await expect(app).toHaveAttribute("data-theme", "dark");
    await expect(app).toHaveClass(/theme-dark/);
    await expect(modeBtn.locator(".control-val")).toHaveText("LIGHT");
  });
});
