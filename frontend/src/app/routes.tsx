import type { RouteObject } from "react-router";
import { platform } from "@/shared/lib/platform";

import { WebLayout } from "@/app/layouts/WebLayout";
import { AppLayout } from "@/app/layouts/AppLayout";
import { RequireAuth, RequireGuest, LoginPage, RegisterPage } from "@/features/auth";

import { LandingPage } from "@/app/pages/LandingPage";
import { NotFoundPage } from "@/app/pages/NotFoundPage";
import { DashboardPage } from "@/features/dashboard/pages/DashboardPage";
import { ProfilePage } from "@/features/profile/pages/ProfilePage";

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
