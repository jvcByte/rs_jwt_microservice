import { useTheme } from "next-themes"
import { Toaster as Sonner, type ToasterProps } from "sonner"
import { CircleCheckIcon, InfoIcon, TriangleAlertIcon, OctagonXIcon, Loader2Icon } from "lucide-react"

const Toaster = ({ ...props }: ToasterProps) => {
  // Use `resolvedTheme` (always "light" | "dark"), not `theme` — `theme` can be
  // "system", which makes sonner resolve darkness from the OS media query
  // instead of the `.dark` class next-themes actually applies. Those diverge as
  // soon as the in-app toggle overrides the OS, leaving toasts on the wrong
  // palette. `resolvedTheme` ties sonner directly to the app's real theme.
  const { resolvedTheme } = useTheme()

  return (
    <Sonner
      theme={(resolvedTheme ?? "system") as ToasterProps["theme"]}
      className="toaster group"
      icons={{
        success: (
          <CircleCheckIcon className="size-4" />
        ),
        info: (
          <InfoIcon className="size-4" />
        ),
        warning: (
          <TriangleAlertIcon className="size-4" />
        ),
        error: (
          <OctagonXIcon className="size-4" />
        ),
        loading: (
          <Loader2Icon className="size-4 animate-spin" />
        ),
      }}
      style={
        {
          // Bind the toast surface to the INVERTED theme tokens so it always
          // contrasts the page instead of blending. --primary auto-switches:
          // near-black on light theme, near-white on dark; --primary-foreground
          // is its readable text. --popover (the old value) equaled --background
          // on light theme, which is exactly why toasts blended into white.
          "--normal-bg": "var(--primary)",
          "--normal-text": "var(--primary-foreground)",
          "--normal-border": "var(--primary)",
          "--border-radius": "var(--radius)",
        } as React.CSSProperties
      }
      toastOptions={{
        classNames: {
          toast: "cn-toast",
        },
      }}
      // Clear the Android status bar / notch: --inset-top is injected natively
      // (0 on web/desktop), + the 16px sonner default gap. Both offset and
      // mobileOffset are set since sonner switches between them at 600px width.
      offset={{ top: "calc(var(--inset-top) + 16px)" }}
      mobileOffset={{ top: "calc(var(--inset-top) + 16px)" }}
      {...props}
    />
  )
}

export { Toaster }
