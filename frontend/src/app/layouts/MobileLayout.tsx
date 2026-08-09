import { Outlet, NavLink } from "react-router";
import { LayoutDashboard, User } from "lucide-react";
import { useAuth } from "@/features/auth";
import { ThemeToggle } from "@/shared/components/ThemeToggle";
import { cn } from "@/shared/lib/utils";

const tabs = [
  { to: "/app", label: "Home", icon: LayoutDashboard, end: true },
  { to: "/app/profile", label: "Profile", icon: User },
];

export function MobileLayout() {
  const { user } = useAuth();

  if (!user) {
    // Auth pages — no chrome, just the form centered in the screen
    return (
      <div
        className="relative flex min-h-screen flex-col"
        style={{ paddingLeft: "var(--inset-left)", paddingRight: "var(--inset-right)" }}
      >
        <div
          className="absolute z-10"
          style={{
            top: "calc(var(--inset-top) + 0.5rem)",
            right: "calc(var(--inset-right) + 0.5rem)",
          }}
        >
          <ThemeToggle />
        </div>
        <main className="flex-1">
          <Outlet />
        </main>
      </div>
    );
  }

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-background">
      {/* Top app bar — background extends behind the status bar; padding-top
          insets only the content row below the tray. */}
      <header
        className="shrink-0 border-b bg-background"
        style={{
          paddingTop: "var(--inset-top)",
          paddingLeft: "var(--inset-left)",
          paddingRight: "var(--inset-right)",
        }}
      >
        <div className="flex h-14 items-center justify-between px-4">
          <span className="font-semibold text-base">AppTemplate</span>
          <ThemeToggle />
        </div>
      </header>

      {/* Scrollable content */}
      <main
        className="flex-1 overflow-y-auto"
        style={{
          paddingLeft: "var(--inset-left)",
          paddingRight: "var(--inset-right)",
        }}
      >
        <Outlet />
      </main>

      {/* Bottom tab bar — .mobile-tab-bar adds bottom inset behind the nav bar */}
      <nav
        className="mobile-tab-bar shrink-0 border-t bg-background"
        style={{
          paddingLeft: "var(--inset-left)",
          paddingRight: "var(--inset-right)",
        }}
      >
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
