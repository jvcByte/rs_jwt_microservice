import { useSyncExternalStore } from "react";

/**
 * Reactive `matchMedia`. Re-renders when the query's match state flips
 * (viewport resize, orientation change). SSR-safe default of `false`.
 */
export function useMediaQuery(query: string): boolean {
  function subscribe(callback: () => void) {
    const mql = window.matchMedia(query);
    mql.addEventListener("change", callback);
    return () => mql.removeEventListener("change", callback);
  }
  const getSnapshot = () => window.matchMedia(query).matches;
  const getServerSnapshot = () => false;

  return useSyncExternalStore(subscribe, getSnapshot, getServerSnapshot);
}

// Tailwind's `md` breakpoint is 768px, so "mobile" is anything below it.
// 767.98px avoids the boundary landing in both queries at exactly 768.
export function useIsMobileViewport(): boolean {
  return useMediaQuery("(max-width: 767.98px)");
}
