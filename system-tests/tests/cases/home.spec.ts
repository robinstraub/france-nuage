import { expect, test } from "../base";

test.describe("Console / Accueil", () => {
  test("la console affiche France-nuage", async ({ pages }) => {
    await pages.home.goto();
    await expect(pages.home.page.getByText("France-nuage")).toBeVisible();
  });
});
