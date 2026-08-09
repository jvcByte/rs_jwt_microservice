const BASE_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";

/**
 * Cross-cutting fetch wrapper shared by every feature's api module. Adds the
 * JSON content-type, optional bearer auth, a 15s abort timeout, and a uniform
 * error shape. Feature api files (e.g. features/auth/api.ts) build their
 * endpoints on top of this — this is the one place networking concerns live.
 */
export async function request<T>(
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
