import type { User } from "oidc-client-ts";
import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import { userManager } from "../auth/user-manager";

export function PageGuard() {
  const [user, setUser] = useState<User | null | undefined>(undefined);

  useEffect(() => {
    userManager.getUser().then((u) => {
      if (u && !u.expired) {
        setUser(u);
      } else {
        userManager.signinRedirect();
      }
    });
  }, []);

  if (user === undefined) {
    return null;
  }

  return <Outlet context={user} />;
}
