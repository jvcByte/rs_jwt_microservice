import { isTauri } from "@tauri-apps/api/core";

export type Platform = "web" | "desktop" | "mobile";

function detectMobile(): boolean {
  return /android|iphone|ipad|ipod/i.test(navigator.userAgent);
}

export function getPlatform(): Platform {
  if (!isTauri()) return "web";
  return detectMobile() ? "mobile" : "desktop";
}

export const platform = getPlatform();
export const isWeb = platform === "web";
export const isDesktop = platform === "desktop";
export const isMobile = platform === "mobile";
export const isApp = isDesktop || isMobile; // running inside Tauri
