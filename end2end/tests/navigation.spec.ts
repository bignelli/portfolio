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
});
