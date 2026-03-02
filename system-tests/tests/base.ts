import { test as base } from "@playwright/test";
import { HomePage } from "./pages/home.page";
import { LoginPage } from "./pages/login.page";

const KEYCLOAK_URL = process.env.KEYCLOAK_URL || "https://keycloak.localhost";
const OIDC_CLIENT_ID = "e2e-tests";
const CONSOLE_OIDC_CLIENT_ID =
	process.env.CONSOLE_OIDC_CLIENT_ID || "console";
const CONSOLE_OIDC_AUTHORITY =
	process.env.CONSOLE_OIDC_AUTHORITY ||
	"https://keycloak.localhost/realms/france-nuage";

interface TokenResponse {
	access_token: string;
	id_token: string;
	refresh_token: string;
	token_type: string;
	expires_in: number;
	scope: string;
}

class KeycloakApi {
	constructor(
		private baseUrl: string,
		private realm: string,
	) {}

	async getToken(username: string, password: string): Promise<TokenResponse> {
		const response = await fetch(
			`${this.baseUrl}/realms/${this.realm}/protocol/openid-connect/token`,
			{
				method: "POST",
				headers: {
					"Content-Type": "application/x-www-form-urlencoded",
				},
				body: new URLSearchParams({
					grant_type: "password",
					client_id: OIDC_CLIENT_ID,
					username,
					password,
					scope: "openid profile email",
				}),
			},
		);

		if (!response.ok) {
			const body = await response.text();
			throw new Error(
				`Keycloak token request failed (${response.status}): ${body}`,
			);
		}

		return response.json();
	}
}

type Pages = {
	home: HomePage;
	login: LoginPage;
};

type WorkerFixtures = {
	keycloak: KeycloakApi;
};

type TestFixtures = {
	pages: Pages;
	actingAs: (credentials?: {
		username: string;
		password: string;
	}) => Promise<void>;
};

export const test = base.extend<TestFixtures, WorkerFixtures>({
	keycloak: [
		async ({}, use) => {
			await use(new KeycloakApi(KEYCLOAK_URL, "france-nuage"));
		},
		{ scope: "worker" },
	],

	pages: async ({ page }, use) => {
		await use({
			home: new HomePage(page),
			login: new LoginPage(page),
		});
	},

	actingAs: async ({ keycloak, page }, use) => {
		await use(async (credentials) => {
			const { username, password } = credentials ?? {
				username: "testuser@france-nuage.test",
				password: "test123",
			};

			const tokens = await keycloak.getToken(username, password);
			const storageKey = `oidc.user:${CONSOLE_OIDC_AUTHORITY}:${CONSOLE_OIDC_CLIENT_ID}`;

			const idTokenPayload = JSON.parse(
				Buffer.from(tokens.id_token.split(".")[1], "base64url").toString(),
			);

			const user = {
				access_token: tokens.access_token,
				id_token: tokens.id_token,
				refresh_token: tokens.refresh_token,
				token_type: tokens.token_type,
				scope: tokens.scope,
				expires_at: Math.floor(Date.now() / 1000) + tokens.expires_in,
				profile: {
					sub: idTokenPayload.sub,
					email: idTokenPayload.email,
					email_verified: idTokenPayload.email_verified,
					name: idTokenPayload.name,
					preferred_username: idTokenPayload.preferred_username,
					given_name: idTokenPayload.given_name,
					family_name: idTokenPayload.family_name,
				},
			};

			await page.addInitScript(
				({ key, value }: { key: string; value: string }) => {
					sessionStorage.setItem(key, value);
				},
				{ key: storageKey, value: JSON.stringify(user) },
			);
		});
	},
});

export { expect } from "@playwright/test";
