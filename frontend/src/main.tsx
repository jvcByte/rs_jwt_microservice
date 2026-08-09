import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createBrowserRouter } from "react-router";
import { ThemeProvider } from "next-themes";
import { Toaster } from "@/shared/components/ui/sonner";
import { TooltipProvider } from "@/shared/components/ui/tooltip";
import { AuthProvider } from "@/features/auth";
import { platform } from "@/shared/lib/platform";
import { routes } from "@/app/routes";
import "@/index.css";

document.documentElement.setAttribute("data-platform", platform);

const router = createBrowserRouter(routes);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem disableTransitionOnChange>
      <AuthProvider>
        <TooltipProvider>
          <RouterProvider router={router} />
          <Toaster position="top-center" />
        </TooltipProvider>
      </AuthProvider>
    </ThemeProvider>
  </React.StrictMode>
);
