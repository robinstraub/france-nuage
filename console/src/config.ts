export const config = {
  api: {
    baseUrl: import.meta.env.VITE_API_BASE_URL ?? "http://localhost:8080",
  },
  oidc: {
    authority:
      import.meta.env.VITE_OIDC_AUTHORITY ??
      "http://localhost:8080/realms/france-nuage",
    clientId: import.meta.env.VITE_OIDC_CLIENT_ID ?? "console",
    redirectUri:
      import.meta.env.VITE_OIDC_REDIRECT_URI ??
      "http://localhost:5173/callback",
  },
};
