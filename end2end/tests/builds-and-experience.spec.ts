import { test, expect } from "@playwright/test";

test.describe("Featured Builds, Career Experience & Contact Matrix", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("should render exactly the 3 standalone featured builds with source links", async ({ page }) => {
    const cards = page.locator("#projects .build-card");
    await expect(cards).toHaveCount(3);

    // Build 1: Portfolio
    const card1 = cards.nth(0);
    await expect(card1.locator(".build-tag")).toHaveText("PRJ_01");
    await expect(card1.locator(".build-title")).toContainText("Rust + Leptos");
    const link1 = card1.locator(".build-link-btn");
    await expect(link1).toBeVisible();
    await expect(link1).toHaveAttribute("target", "_blank");
    await expect(link1).toHaveAttribute("rel", "noopener noreferrer");

    // Build 2: KregBot
    const card2 = cards.nth(1);
    await expect(card2.locator(".build-tag")).toHaveText("PRJ_02");
    await expect(card2.locator(".build-title")).toContainText("KregBot");
    const link2 = card2.locator(".build-link-btn");
    await expect(link2).toBeVisible();
    await expect(link2).toHaveAttribute("href", "https://github.com/FarraindesLokoes/KregBot");

    // Build 3: Minecraft Mods
    const card3 = cards.nth(2);
    await expect(card3.locator(".build-tag")).toHaveText("PRJ_03");
    await expect(card3.locator(".build-title")).toContainText("Minecraft Mods");
    const link3 = card3.locator(".build-link-btn");
    await expect(link3).toBeVisible();
    await expect(link3).toHaveAttribute("href", "https://github.com/bignelli");
  });

  test("should render the 4 verified career experience entries", async ({ page }) => {
    const logCards = page.locator("#experience .log-card");
    await expect(logCards).toHaveCount(4);

    // Tecpar
    await expect(logCards.nth(0).locator(".log-company")).toContainText("Tecpar");
    await expect(logCards.nth(0).locator(".log-period")).toContainText("Mar 2026");

    // Fall Engenharia
    await expect(logCards.nth(1).locator(".log-company")).toContainText("Fall Engenharia");
    await expect(logCards.nth(1).locator(".log-period")).toContainText("Feb 2025");

    // Solvefin
    await expect(logCards.nth(2).locator(".log-company")).toContainText("Solvefin");
    await expect(logCards.nth(2).locator(".log-period")).toContainText("May 2024");

    // ZF Friedrichshafen
    await expect(logCards.nth(3).locator(".log-company")).toContainText("ZF Friedrichshafen");
    await expect(logCards.nth(3).locator(".log-period")).toContainText("Mar 2021");
  });

  test("should render contact matrix with direct communication channels", async ({ page }) => {
    const contactMatrix = page.locator("#contact .contact-matrix");
    await expect(contactMatrix).toBeVisible();

    // Email
    const emailLink = contactMatrix.locator('a[href^="mailto:"]');
    await expect(emailLink).toHaveAttribute("href", "mailto:guilhermebignelli@gmail.com");

    // LinkedIn
    const linkedinLink = contactMatrix.locator('a[href*="linkedin.com"]');
    await expect(linkedinLink).toHaveAttribute("target", "_blank");
    await expect(linkedinLink).toHaveAttribute("rel", "noopener noreferrer");

    // GitHub
    const githubLink = contactMatrix.locator('a[href="https://github.com/bignelli"]');
    await expect(githubLink).toHaveAttribute("target", "_blank");

    // Phone
    const phoneLink = contactMatrix.locator('a[href^="tel:"]');
    await expect(phoneLink).toHaveAttribute("href", "tel:+5541987406606");
  });
});
