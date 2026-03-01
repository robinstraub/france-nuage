import type { Locator } from "@playwright/test";
import { BasePage } from "./base.page";

export class HomePage extends BasePage {
	get header(): Locator {
		return this.page.locator("header");
	}

	get sidebar(): Locator {
		return this.page.locator("nav[aria-label='Navigation principale']");
	}

	get userName(): Locator {
		return this.header.locator("text=Test User");
	}

	get logoutButton(): Locator {
		return this.page.getByLabel("Se déconnecter");
	}

	async goto() {
		await super.goto("/");
	}
}
