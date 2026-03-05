import { expect, test } from "../base";

test.describe("Story 1.3 : Shell de la console et authentification frontend", () => {
	test("un utilisateur non authentifié ne voit pas le shell", async ({
		pages,
	}) => {
		await pages.home.goto();
		// Le shell ne doit pas être visible : le PageGuard bloque le rendu
		// et tente une redirection OIDC (non vérifiable en Docker car
		// localhost:8080 n'est pas accessible depuis le container)
		await expect(pages.home.header).not.toBeVisible();
	});

	test.describe("utilisateur authentifié", () => {
		test.beforeEach(async ({ actingAs, pages }) => {
			await actingAs();
			await pages.onboarding.goto();
			await pages.onboarding.createOrganization("Test Org");
		});

		test("un utilisateur authentifié voit le shell de la console", async ({
			pages,
		}) => {
			await pages.home.goto();
			await expect(pages.home.header).toBeVisible();
			await expect(
				pages.home.page.getByRole("main"),
			).toBeVisible();
		});

		test("le nom de l'utilisateur apparaît dans le header", async ({
			pages,
		}) => {
			await pages.home.goto();
			await expect(pages.home.userName("Test User")).toBeVisible();
		});

		test("le bouton de déconnexion est visible", async ({ pages }) => {
			await pages.home.goto();
			await expect(pages.home.logoutButton).toBeVisible();
		});

		test("le shell est accessible (WCAG AA)", async ({ pages }) => {
			await pages.home.goto();
			await pages.home.checkAccessibility();
		});
	});
});
