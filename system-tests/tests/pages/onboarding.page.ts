import { expect, type Locator } from "@playwright/test";
import { BasePage } from "./base.page";

export class OnboardingPage extends BasePage {
	get heading(): Locator {
		return this.page.getByRole("heading", {
			name: "Bienvenue sur France-nuage",
		});
	}

	get nameInput(): Locator {
		return this.page.getByPlaceholder("Nom de l'organisation");
	}

	get submitButton(): Locator {
		return this.page.getByRole("button", {
			name: "Créer l'organisation",
		});
	}

	async goto() {
		await super.goto("/onboarding");
	}

	async createOrganization(name: string) {
		await this.nameInput.fill(name);
		await this.submitButton.click();
	}

	async expectHeadingVisible() {
		await expect(this.heading).toBeVisible();
	}
}
