import { request } from "@/shared/api/client";

export interface TokenResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
  refresh_token?: string;
  user?: { id: string; name: string; email: string };
}

export interface UserResponse {
  id: string;
  name: string;
  email: string;
}

/**
 * Auth endpoints, namespaced by feature and built on the shared request()
 * client. New features add their own `features/<x>/api.ts` the same way.
 */
export const authApi = {
  register: (name: string, email: string, password: string) =>
    request<TokenResponse>("/api/auth/register", {
      method: "POST",
      body: JSON.stringify({ name, email, password }),
    }),

  login: (email: string, password: string) =>
    request<TokenResponse>("/api/auth/login", {
      method: "POST",
      body: JSON.stringify({ email, password }),
    }),

  refresh: (refresh_token: string) =>
    request<TokenResponse>("/api/auth/refresh", {
      method: "POST",
      body: JSON.stringify({ refresh_token }),
    }),

  logout: (refresh_token: string, token: string) =>
    request<void>("/api/auth/logout", {
      method: "POST",
      body: JSON.stringify({ refresh_token }),
    }, token),

  me: (token: string) =>
    request<UserResponse>("/api/auth/me", {}, token),
};
