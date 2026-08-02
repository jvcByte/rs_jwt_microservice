import { Outlet, NavLink, useNavigate } from "react-router";
import { LayoutDashboard, User } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";
import { cn } from "@/lib/utils";

const tabs = [
  { to: "/app", label: "Home", icon: LayoutDashboard, end: true },
  { to: "/app/profile", label: "Profile", icon: User },
];

export function MobileLayout() {
  const { user } = useAuth();

  if (!user) {
    // Auth pages — no chrome, just the form centered in the screen
    return (
      <div className="flex min-h-screen flex-col">
        <main className="flex-1">
          <Outlet />
        </main>
      </div>
    );
  }

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-background">
      {/* Top app bar */}
      <header className="flex h-14 shrink-0 items-center border-b bg-background px-4">
        <span className="font-semibold text-base">AppTemplate</span>
      </header>

      {/* Scrollable content */}
      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>

      {/* Bottom tab bar */}
      <nav className="mobile-tab-bar shrink-0 border-t bg-background">
        <div className="flex h-14 items-center justify-around">
          {tabs.map(({ to, label, icon: Icon, end }) => (
            <NavLink
              key={to}
              to={to}
              end={end}
              className={({ isActive }) =>
                cn(
                  "flex flex-col items-center gap-0.5 px-6 py-1 text-xs text-muted-foreground transition-colors",
                  isActive && "text-primary"
                )
              }
            >
              {({ isActive }) => (
                <>
                  <Icon size={22} strokeWidth={isActive ? 2.5 : 1.8} />
                  <span>{label}</span>
                </>
              )}
            </NavLink>
          ))}
        </div>
      </nav>
    </div>
  );
}
