import { UserManager, WebStorageStateStore } from "oidc-client-ts";
import { config } from "../config";

export const userManager = new UserManager({
  authority: config.oidc.authority,
  client_id: config.oidc.clientId,
  redirect_uri: config.oidc.redirectUri,
  post_logout_redirect_uri: window.location.origin,
  response_type: "code",
  scope: "openid profile email",
  automaticSilentRenew: true,
  userStore: new WebStorageStateStore({ store: sessionStorage }),
});
