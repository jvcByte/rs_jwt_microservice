const BASE_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";

interface TokenResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
  refresh_token?: string;
  user?: { id: string; name: string; email: string };
}

interface UserResponse {
  id: string;
  name: string;
  email: string;
}

async function request<T>(
  path: string,
  options: RequestInit = {},
  token?: string
): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(options.headers as Record<string, string>),
  };
  if (token) headers["Authorization"] = `Bearer ${token}`;

  // Without a timeout, an unreachable host makes fetch wait for the OS TCP
  // timeout (can be minutes) — long enough to wedge session restore and leave
  // the app stuck on a blank loading state. Abort after 15s instead.
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 15_000);
  try {
    const res = await fetch(`${BASE_URL}${path}`, {
      ...options,
      headers,
      signal: controller.signal,
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({ error: res.statusText }));
      throw new Error(err.error ?? "Request failed");
    }
    if (res.status === 204) return undefined as T;
    return res.json();
  } catch (e) {
    if (e instanceof DOMException && e.name === "AbortError") {
      throw new Error("Network request timed out");
    }
    throw e;
  } finally {
    clearTimeout(timer);
  }
}

export const api = {
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
