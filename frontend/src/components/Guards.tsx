import { Navigate, Outlet } from "react-router";
import { useAuth } from "@/hooks/useAuth";
import { platform } from "@/lib/platform";

/** Redirects to login if not authenticated. */
export function RequireAuth() {
  const { user, isLoading } = useAuth();
  if (isLoading) return null; // wait for session restore
  if (!user) return <Navigate to={platform === "web" ? "/login" : "/"} replace />;
  return <Outlet />;
}

/** Redirects to /app if already authenticated (prevents showing login to logged-in users). */
export function RequireGuest() {
  const { user, isLoading } = useAuth();
  if (isLoading) return null;
  if (user) return <Navigate to="/app" replace />;
  return <Outlet />;
}
