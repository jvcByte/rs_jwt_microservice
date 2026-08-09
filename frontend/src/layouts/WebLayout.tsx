import { Outlet, Link, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { ThemeToggle } from "@/components/ThemeToggle";

export function WebLayout() {
  const { pathname } = useLocation();
  const isAuthPage = pathname === "/login" || pathname === "/register";

  return (
    <div className="relative min-h-screen flex flex-col">
      {/* Top nav — hidden on auth pages */}
      {!isAuthPage && (
        <header className="sticky top-0 z-50 w-full border-b bg-background/80 backdrop-blur">
          <div className="container mx-auto flex h-14 items-center justify-between px-4">
            <Link to="/" className="font-semibold text-lg tracking-tight">
              AppTemplate
            </Link>
            <nav className="flex items-center gap-2">
              <ThemeToggle />
              <Button variant="ghost" size="sm" asChild>
                <Link to="/login">Sign in</Link>
              </Button>
              <Button size="sm" asChild>
                <Link to="/register">Get started</Link>
              </Button>
            </nav>
          </div>
        </header>
      )}
      {isAuthPage && (
        <div className="absolute right-3 top-3 z-10">
          <ThemeToggle />
        </div>
      )}
      <main className="flex-1">
        <Outlet />
      </main>
      {!isAuthPage && (
        <footer className="border-t py-6 text-center text-xs text-muted-foreground">
          © {new Date().getFullYear()} AppTemplate. Built with Rust + React.
        </footer>
      )}
    </div>
  );
}
