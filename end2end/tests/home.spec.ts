import { test, expect } from "@playwright/test";

test.describe("Homepage & Hero Section", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("should display correct page title and initial metadata", async ({ page }) => {
    await expect(page).toHaveTitle(/Guilherme Bignelli/);
  });

  test("should render the navbar with brand and links", async ({ page }) => {
    const brand = page.locator(".nav-brand");
    await expect(brand).toBeVisible();
    await expect(brand).toContainText("guilherme.bignelli");

    const links = page.locator(".nav-links .nav-link");
    await expect(links).toHaveCount(5);
  });

  test("should render hero heading, subtitle and status indicator", async ({ page }) => {
    const heroTitle = page.locator(".hero-title");
    await expect(heroTitle).toContainText("Guilherme");
    await expect(heroTitle).toContainText("Bignelli");

    const subtitle = page.locator(".hero-subtitle");
    await expect(subtitle).toBeVisible();
    await expect(subtitle).toContainText("Backend Software Engineer");

    const statusText = page.locator(".status-text");
    await expect(statusText).toBeVisible();
    await expect(statusText).toContainText("READY");
  });

  test("should display terminal specs box with technical stack", async ({ page }) => {
    const specBox = page.locator(".terminal-spec-box");
    await expect(specBox).toBeVisible();

    await expect(specBox).toContainText("CORE");
    await expect(specBox).toContainText("Rust · Python · Java · C/C++");

    await expect(specBox).toContainText("DATA");
    await expect(specBox).toContainText("Airflow · Selenium · Pandas · SQL");

    await expect(specBox).toContainText("SYSTEMS");
    await expect(specBox).toContainText("Linux Internals · CAN Bus · Kafka · GCP");
  });

  test("should render hero action buttons with valid links", async ({ page }) => {
    const contactBtn = page.locator('.hero-actions a[href="#contact"]');
    await expect(contactBtn).toBeVisible();

    const githubBtn = page.locator('.hero-actions a[href="https://github.com/bignelli"]');
    await expect(githubBtn).toBeVisible();
    await expect(githubBtn).toHaveAttribute("target", "_blank");
    await expect(githubBtn).toHaveAttribute("rel", "noopener noreferrer");

    const linkedinBtn = page.locator('.hero-actions a[href*="linkedin.com"]');
    await expect(linkedinBtn).toBeVisible();
    await expect(linkedinBtn).toHaveAttribute("target", "_blank");
  });

  test("should render photo frame with technical coordinates", async ({ page }) => {
    const frame = page.locator(".photo-frame");
    await expect(frame).toBeVisible();
    await expect(frame.locator(".frame-id")).toContainText("IMG_REF: GB_DEV");
    await expect(frame.locator(".frame-coords")).toContainText("LAT -25.4284 // LON -49.2733");
  });

  test("should render all 5 numbered portfolio sections and footer", async ({ page }) => {
    await expect(page.locator("#about")).toBeVisible();
    await expect(page.locator("#skills")).toBeVisible();
    await expect(page.locator("#experience")).toBeVisible();
    await expect(page.locator("#projects")).toBeVisible();
    await expect(page.locator("#contact")).toBeVisible();
    await expect(page.locator("footer.footer")).toBeVisible();
  });
});
