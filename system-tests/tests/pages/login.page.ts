import { BasePage } from "./base.page";

export class LoginPage extends BasePage {
	get usernameField() {
		return this.page.locator("#username");
	}

	get passwordField() {
		return this.page.locator("#password");
	}

	get loginButton() {
		return this.page.locator("#kc-login");
	}

	async goto() {
		await super.goto("/");
	}

	async login(username: string, password: string) {
		await this.usernameField.fill(username);
		await this.passwordField.fill(password);
		await this.loginButton.click();
	}
}
