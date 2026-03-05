import { test } from "../base";

test.describe("Console / Organisations", () => {
	test.beforeEach(async ({ actingAs }) => {
		await actingAs();
	});

	test("rediriger vers onboarding quand aucune organisation", async ({
		pages,
	}) => {
		await pages.home.goto();
		await pages.onboarding.expectHeadingVisible();
	});

	test("creer une organisation depuis la page onboarding", async ({
		pages,
	}) => {
		await pages.onboarding.goto();
		await pages.onboarding.createOrganization("Mon Organisation E2E");
		await pages.home.expectApplicationsVisible();
	});

	test("afficher le switcher d'organisations dans le header", async ({
		pages,
	}) => {
		await pages.onboarding.goto();
		await pages.onboarding.createOrganization("Organisation Test");
		await pages.home.expectOrganizationVisible("Organisation Test");
	});
});
