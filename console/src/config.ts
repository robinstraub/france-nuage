export const config = {
  api: {
    baseUrl:
      import.meta.env.VITE_API_BASE_URL ?? "https://controlplane.localhost",
  },
  oidc: {
    authority:
      import.meta.env.VITE_OIDC_AUTHORITY ??
      "https://keycloak.localhost/realms/france-nuage",
    clientId: import.meta.env.VITE_OIDC_CLIENT_ID ?? "console",
    redirectUri:
      import.meta.env.VITE_OIDC_REDIRECT_URI ??
      "https://console.localhost/callback",
  },
};
