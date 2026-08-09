# Features

This app is organized **feature-first**. A feature folder owns everything that
makes it work — its `api.ts`, `pages/`, `components/`, hooks, and context — so
the whole feature can be reasoned about, moved, or deleted as a unit.

`auth/` is the worked example of the convention:

```
features/auth/
  index.ts           # public surface — what the rest of the app may import
  api.ts             # authApi, built on shared/api/client.ts
  AuthContext.tsx    # provider + useAuth definition
  useAuth.ts         # re-export of the hook
  Guards.tsx         # RequireAuth / RequireGuest route guards
  components/         # used ONLY by auth (e.g. PasswordInput)
  pages/             # LoginPage, RegisterPage
```

## Rules

- **Expose a public surface via `index.ts`.** Other features and the app shell
  import from `@/features/<name>`, never from a feature's internal files.
- **Import your own internals relatively** (`./AuthContext`, `../useAuth`) to
  avoid a cycle through your own barrel.
- **Code used by 2+ features goes in `shared/`.** `ThemeToggle` lives in
  `shared/components` because every layout uses it; `PasswordInput` lives in
  `auth/components` because only auth uses it.
- **Each feature adds its own `api.ts`** on top of the one cross-cutting
  `shared/api/client.ts` (the `request()` wrapper). Don't grow a single flat
  api file.

## Adding a feature

1. Create `features/<name>/` with `pages/` (and `api.ts`, `components/`,
   context, hooks as needed).
2. Add a `<name>/api.ts` importing `request` from `@/shared/api/client` if it
   talks to the backend.
3. Export the public surface from `features/<name>/index.ts`.
4. Wire its pages into `app/routes.tsx`.
