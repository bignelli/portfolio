import { test, expect } from "@playwright/test";

test.describe("Navigation & Responsive Menu", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("should have correct href targets on desktop navbar links", async ({ page }) => {
    await expect(page.locator('.nav-link[href="#about"]')).toBeVisible();
    await expect(page.locator('.nav-link[href="#skills"]')).toBeVisible();
    await expect(page.locator('.nav-link[href="#experience"]')).toBeVisible();
    await expect(page.locator('.nav-link[href="#projects"]')).toBeVisible();
    await expect(page.locator('.nav-link[href="#contact"]')).toBeVisible();
  });

  test("should show hamburger menu and toggle nav-links on mobile viewport", async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    const hamburger = page.locator(".nav-hamburger");
    await expect(hamburger).toBeVisible();

    const navLinks = page.locator(".nav-links");
    // Initially not active
    await expect(navLinks).not.toHaveClass(/active/);

    // Click hamburger to open
    await hamburger.click();
    await expect(navLinks).toHaveClass(/active/);

    // Click link inside menu to navigate and auto-close
    const aboutLink = page.locator('.nav-link[href="#about"]');
    await aboutLink.click();
    await expect(navLinks).not.toHaveClass(/active/);
  });

  test("should fit all navbar controls and brand within viewport on mobile without horizontal scroll", async ({ page }) => {
    // Set narrow mobile viewport (e.g. 360px wide)
    await page.setViewportSize({ width: 360, height: 740 });

    const navControls = page.locator(".nav-controls");
    await expect(navControls).toBeVisible();

    const modeBtn = page.locator('.control-btn:has(.control-val:text-matches("LIGHT|DARK"))');
    await expect(modeBtn).toBeVisible();

    const langBtn = page.locator('.control-btn:has(.control-val:text-matches("PT|EN"))');
    await expect(langBtn).toBeVisible();

    const hamburger = page.locator(".nav-hamburger");
    await expect(hamburger).toBeVisible();

    // Verify all control buttons fit entirely within the 360px viewport
    const hamburgerBox = await hamburger.boundingBox();
    expect(hamburgerBox).not.toBeNull();
    if (hamburgerBox) {
      expect(hamburgerBox.x + hamburgerBox.width).toBeLessThanOrEqual(360);
    }

    // Verify no horizontal document overflow
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth);
  });
});
