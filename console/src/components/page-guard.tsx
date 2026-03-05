import type { User } from "oidc-client-ts";
import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { userManager } from "../auth/user-manager";
import { fetchOrganizations } from "../features/organizations/organizations-slice";
import type { AppDispatch, RootState } from "../store";

export function PageGuard() {
  const [user, setUser] = useState<User | null | undefined>(undefined);
  const dispatch = useDispatch<AppDispatch>();
  const { items, status } = useSelector(
    (state: RootState) => state.organizations,
  );
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    userManager
      .getUser()
      .then((u) => {
        if (u && !u.expired) {
          setUser(u);
          dispatch(fetchOrganizations());
        } else {
          return userManager.signinRedirect();
        }
      })
      .catch(() => {
        userManager.signinRedirect().catch(() => {
          setUser(null);
        });
      });
  }, [dispatch]);

  useEffect(() => {
    if (!user || status === "idle" || status === "loading") return;

    if (location.pathname === "/onboarding") return;

    if (status === "failed" || items.length === 0) {
      navigate("/onboarding", { replace: true });
    }
  }, [user, items, status, location.pathname, navigate]);

  if (!user) {
    return null;
  }

  return <Outlet context={user} />;
}
