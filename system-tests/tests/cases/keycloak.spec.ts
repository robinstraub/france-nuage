import { expect, test } from "../base";

test.describe("Story 1.2: Keycloak authentication", () => {
	test("le token Keycloak est valide pour l'utilisateur de test", async ({
		keycloak,
	}) => {
		const tokens = await keycloak.getToken(
			"testuser@france-nuage.test",
			"test123",
		);

		expect(tokens.access_token).toBeTruthy();
		expect(tokens.id_token).toBeTruthy();
		expect(tokens.token_type).toBe("Bearer");
		expect(tokens.scope).toContain("openid");
	});

	test("le token contient les informations du profil utilisateur", async ({
		keycloak,
	}) => {
		const tokens = await keycloak.getToken(
			"testuser@france-nuage.test",
			"test123",
		);

		const payload = JSON.parse(
			Buffer.from(tokens.id_token.split(".")[1], "base64url").toString(),
		);

		expect(payload.email).toBe("testuser@france-nuage.test");
		expect(payload.email_verified).toBe(true);
		expect(payload.sub).toBeTruthy();
	});
});
