import { test as base, type Page } from "@playwright/test";

class HomePage {
  constructor(public readonly page: Page) {}

  async goto() {
    await this.page.goto("/");
  }
}

type Pages = {
  home: HomePage;
};

type Fixtures = {
  pages: Pages;
};

export const test = base.extend<Fixtures>({
  pages: async ({ page }, use) => {
    await use({
      home: new HomePage(page),
    });
  },
});

export { expect } from "@playwright/test";
