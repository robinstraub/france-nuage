import AxeBuilder from "@axe-core/playwright";
import type { Page } from "@playwright/test";

export abstract class BasePage {
	constructor(public readonly page: Page) {}

	async goto(path = "/") {
		await this.page.goto(path);
	}

	async checkAccessibility() {
		return new AxeBuilder({ page: this.page })
			.withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
			.analyze();
	}
}
