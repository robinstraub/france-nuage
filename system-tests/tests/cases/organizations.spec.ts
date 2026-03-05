import { expect, test } from "../base";

test.describe("Console / Organisations", () => {
	test.beforeEach(async ({ actingAs }) => {
		await actingAs();
	});

	test("rediriger vers onboarding quand aucune organisation", async ({
		pages,
	}) => {
		await pages.home.goto();
		await expect(pages.onboarding.heading).toBeVisible();
	});

	test("creer une organisation depuis la page onboarding", async ({
		pages,
	}) => {
		await pages.onboarding.goto();
		await pages.onboarding.createOrganization("Mon Organisation E2E");
		await expect(
			pages.home.page.getByText("Mes applications"),
		).toBeVisible();
	});

	test("afficher le switcher d'organisations dans le header", async ({
		pages,
	}) => {
		await pages.onboarding.goto();
		await pages.onboarding.createOrganization("Organisation Test");
		await expect(
			pages.home.page.getByText("Organisation Test"),
		).toBeVisible();
	});
});
