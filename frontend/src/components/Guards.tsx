import { Navigate, Outlet } from "react-router";
import { Loader2Icon } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";
import { platform } from "@/lib/platform";

/** Full-screen spinner shown while the session is being restored. */
function AuthLoading() {
  return (
    <div className="flex min-h-screen items-center justify-center">
      <Loader2Icon className="size-6 animate-spin text-muted-foreground" />
    </div>
  );
}

/** Redirects to login if not authenticated. */
export function RequireAuth() {
  const { user, isLoading } = useAuth();
  if (isLoading) return <AuthLoading />; // wait for session restore
  if (!user) return <Navigate to={platform === "web" ? "/login" : "/"} replace />;
  return <Outlet />;
}

/** Redirects to /app if already authenticated (prevents showing login to logged-in users). */
export function RequireGuest() {
  const { user, isLoading } = useAuth();
  if (isLoading) return <AuthLoading />;
  if (user) return <Navigate to="/app" replace />;
  return <Outlet />;
}
