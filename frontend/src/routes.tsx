import type { RouteObject } from "react-router";
import { platform } from "@/lib/platform";

import { WebLayout } from "@/layouts/WebLayout";
import { AppLayout } from "@/layouts/AppLayout";
import { RequireAuth, RequireGuest } from "@/components/Guards";

import { LandingPage } from "@/pages/LandingPage";
import { LoginPage } from "@/pages/LoginPage";
import { RegisterPage } from "@/pages/RegisterPage";
import { DashboardPage } from "@/pages/DashboardPage";
import { ProfilePage } from "@/pages/ProfilePage";
import { NotFoundPage } from "@/pages/NotFoundPage";

export const routes: RouteObject[] = [
  // ── Web: landing page + public auth routes ──────────────────────────────
  ...(platform === "web"
    ? [
        {
          element: <WebLayout />,
          children: [
            { path: "/", element: <LandingPage /> },
            {
              element: <RequireGuest />,
              children: [
                { path: "/login", element: <LoginPage /> },
                { path: "/register", element: <RegisterPage /> },
              ],
            },
          ],
        },
      ]
    : []),

  // ── App (desktop/mobile): auth routes at root, no landing page ──────────
  ...(platform !== "web"
    ? [
        {
          element: <AppLayout />,
          children: [
            {
              element: <RequireGuest />,
              children: [
                { path: "/", element: <LoginPage /> },
                { path: "/register", element: <RegisterPage /> },
              ],
            },
          ],
        },
      ]
    : []),

  // ── Protected app routes (all platforms) ────────────────────────────────
  {
    path: "/app",
    element: <AppLayout />,
    children: [
      {
        element: <RequireAuth />,
        children: [
          { index: true, element: <DashboardPage /> },
          { path: "profile", element: <ProfilePage /> },
        ],
      },
    ],
  },

  { path: "*", element: <NotFoundPage /> },
];
