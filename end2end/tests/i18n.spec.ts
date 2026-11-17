import { test, expect } from "@playwright/test";

test.describe("Bilingual Localization (English / Portuguese)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("should render in English by default", async ({ page }) => {
    await expect(page).toHaveTitle("Guilherme Bignelli // Software Engineer");

    const langBtn = page.locator('.control-btn:has-text("LANG:")');
    await expect(langBtn.locator(".control-val")).toHaveText("PT");

    // Check English navbar links
    const navLinks = page.locator(".nav-links .nav-link");
    await expect(navLinks.nth(0)).toContainText("about");
    await expect(navLinks.nth(1)).toContainText("skills");
    await expect(navLinks.nth(2)).toContainText("experience");
    await expect(navLinks.nth(3)).toContainText("projects");
    await expect(navLinks.nth(4)).toContainText("contact");

    // Check English section titles
    await expect(page.locator("#about .section-title")).toHaveText("ABOUT");
    await expect(page.locator("#skills .section-title")).toHaveText("TECHNICAL ARSENAL");
    await expect(page.locator("#experience .section-title")).toHaveText("WORK EXPERIENCE");
    await expect(page.locator("#projects .section-title")).toHaveText("FEATURED BUILDS");
    await expect(page.locator("#contact .section-title")).toHaveText("CONTACT // DISPATCH");
  });

  test("should switch all components to Portuguese when language button is clicked", async ({ page }) => {
    const langBtn = page.locator('.control-btn:has-text("LANG:")');
    await langBtn.click();

    // Check document title
    await expect(page).toHaveTitle("Guilherme Bignelli // Engenheiro de Software");
    await expect(langBtn.locator(".control-val")).toHaveText("EN");

    // Check Portuguese navbar links
    const navLinks = page.locator(".nav-links .nav-link");
    await expect(navLinks.nth(0)).toContainText("sobre");
    await expect(navLinks.nth(1)).toContainText("skills");
    await expect(navLinks.nth(2)).toContainText("experiência");
    await expect(navLinks.nth(3)).toContainText("projetos");
    await expect(navLinks.nth(4)).toContainText("contato");

    // Check Portuguese hero content
    await expect(page.locator(".hero-subtitle")).toHaveText("Engenheiro de Software Backend");
    await expect(page.locator(".status-text")).toHaveText("DISPONÍVEL");

    // Check Portuguese section titles
    await expect(page.locator("#about .section-title")).toHaveText("SOBRE");
    await expect(page.locator("#skills .section-title")).toHaveText("HABILIDADES TÉCNICAS");
    await expect(page.locator("#experience .section-title")).toHaveText("EXPERIÊNCIA PROFISSIONAL");
    await expect(page.locator("#projects .section-title")).toHaveText("PROJETOS EM DESTAQUE");
    await expect(page.locator("#contact .section-title")).toHaveText("CONTATO // CONEXÃO");
  });

  test("should toggle back to English seamlessly", async ({ page }) => {
    const langBtn = page.locator('.control-btn:has-text("LANG:")');

    // Switch to PT
    await langBtn.click();
    await expect(langBtn.locator(".control-val")).toHaveText("EN");
    await expect(page.locator("#about .section-title")).toHaveText("SOBRE");

    // Switch back to EN
    await langBtn.click();
    await expect(langBtn.locator(".control-val")).toHaveText("PT");
    await expect(page.locator("#about .section-title")).toHaveText("ABOUT");
    await expect(page).toHaveTitle("Guilherme Bignelli // Software Engineer");
  });
});
