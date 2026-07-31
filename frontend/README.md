# peeng — frontend

UI that consumes the backend REST API. Built with Tauri v2, React 19, and shadcn/ui — ships on Windows, macOS, Linux, iOS, and Android from a single codebase.

## Tech Stack

| Concern | Tool |
|---|---|
| Desktop / mobile shell | Tauri v2 |
| UI framework | React 19 + TypeScript |
| Build tool | Vite 7 |
| UI components | shadcn/ui + Radix UI |
| Styling | Tailwind CSS v4 |
| Routing | React Router v7 |
| State management | Zustand |
| Data fetching | TanStack Query v5 |
| HTTP client | Axios |
| Icons | Lucide React |
| Package manager | pnpm |

## Project Structure

```
frontend/
  src/
    App.tsx               # Route definitions
    main.tsx              # App entry — providers (QueryClient, Router, Tooltip)
    layouts/
      DashboardLayout.tsx # Collapsible sidebar + outlet
    pages/
      InboxPage.tsx       # Unified notifications inbox
      ListeningPage.tsx   # Keyword / hashtag monitoring
      SchedulerPage.tsx   # Post composer and scheduler
      AccountsPage.tsx    # Connected social accounts
    components/
      ui/                 # shadcn/ui components (owned, not a dependency)
    hooks/                # Custom React hooks
    lib/
      utils.ts            # cn() helper (clsx + tailwind-merge)
  src-tauri/              # Tauri Rust backend (IPC commands, app config)
```

## Prerequisites

- Rust (stable via rustup)
- Node.js 20+
- pnpm (`npm install -g pnpm`)

### Linux system dependencies

```bash
sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev libglib2.0-dev \
  pkg-config build-essential libssl-dev libxdo-dev
```

## Development

```bash
pnpm install
pnpm tauri dev
```

Vite starts on `http://localhost:1420` and the Tauri window opens automatically.

## Build

```bash
# Desktop (current platform)
pnpm tauri build

# Android
pnpm tauri android init   # first time only
pnpm tauri android build

# iOS (macOS only)
pnpm tauri ios init       # first time only
pnpm tauri ios build
```

## Adding shadcn Components

```bash
pnpm dlx shadcn@latest add <component-name>
```

Components are copied into `src/components/ui/`.
