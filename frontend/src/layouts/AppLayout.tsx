import { platform } from "@/lib/platform";
import { DesktopLayout } from "./DesktopLayout";
import { MobileLayout } from "./MobileLayout";

/**
 * Switches between desktop sidebar layout and mobile tab layout
 * based on the platform detected at startup.
 */
export function AppLayout() {
  if (platform === "mobile") return <MobileLayout />;
  return <DesktopLayout />;
}
