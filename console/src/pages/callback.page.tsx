import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { userManager } from "../auth/user-manager";

export function CallbackPage() {
  const navigate = useNavigate();

  useEffect(() => {
    userManager
      .signinCallback()
      .then(() => navigate("/", { replace: true }))
      .catch((error) => {
        console.error("OIDC callback failed:", error);
        navigate("/", { replace: true });
      });
  }, [navigate]);

  return null;
}
