import { Outlet, NavLink, useNavigate } from "react-router";import { LayoutDashboard, User, LogOut } from "lucide-react";
import { Separator } from "@/shared/components/ui/separator";
import { Avatar, AvatarFallback } from "@/shared/components/ui/avatar";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/shared/components/ui/tooltip";
import { Button } from "@/shared/components/ui/button";
import { useAuth } from "@/features/auth";
import { ThemeToggle } from "@/shared/components/ThemeToggle";
import { cn } from "@/shared/lib/utils";
import { toast } from "sonner";

const navItems = [
  { to: "/app", label: "Dashboard", icon: LayoutDashboard, end: true },
  { to: "/app/profile", label: "Profile", icon: User },
];

export function DesktopLayout() {
  const { user } = useAuth();

  // Auth pages (login/register) render chrome-free — no sidebar.
  if (!user) {
    return (
      <div className="relative min-h-screen w-screen overflow-y-auto bg-background">
        <div className="absolute right-3 top-3 z-10">
          <ThemeToggle />
        </div>
        <Outlet />
      </div>
    );
  }

  const initials = user?.name
    ?.split(" ")
    .map((w) => w[0])
    .join("")
    .toUpperCase()
    .slice(0, 2) ?? "??";

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-background">
      {/* Sidebar */}
      <aside className="flex w-14 flex-col items-center border-r bg-sidebar py-4 gap-1">
        <div className="mb-4 flex h-8 w-8 items-center justify-center rounded-lg bg-primary text-primary-foreground text-xs font-bold select-none">
          A
        </div>
        <Separator className="mb-2 w-8" />

        {navItems.map(({ to, label, icon: Icon, end }) => (
          <Tooltip key={to}>
            <TooltipTrigger asChild>
              <NavLink
                to={to}
                end={end}
                className={({ isActive }) =>
                  cn(
                    "flex h-9 w-9 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
                    isActive && "bg-sidebar-accent text-sidebar-accent-foreground"
                  )
                }
              >
                <Icon size={18} />
                <span className="sr-only">{label}</span>
              </NavLink>
            </TooltipTrigger>
            <TooltipContent side="right">{label}</TooltipContent>
          </Tooltip>
        ))}

        <div className="mt-auto flex flex-col items-center gap-2">
          <ThemeToggle />
          <LogoutButton />
          <Avatar className="h-8 w-8 cursor-default">
            <AvatarFallback className="text-xs">{initials}</AvatarFallback>
          </Avatar>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>
    </div>
  );
}

function LogoutButton() {
  const { logout } = useAuth();
  const navigate = useNavigate();

  async function handleLogout() {
    await logout();
    toast.success("Signed out");
    navigate("/", { replace: true });
  }

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          variant="ghost"
          size="icon"
          className="h-9 w-9 text-muted-foreground"
          onClick={handleLogout}
        >
          <LogOut size={18} />
          <span className="sr-only">Sign out</span>
        </Button>
      </TooltipTrigger>
      <TooltipContent side="right">Sign out</TooltipContent>
    </Tooltip>
  );
}
