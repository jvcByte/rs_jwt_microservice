# Database

SeaORM entity definitions and database migrations. Shared by the backend at runtime — the backend runs pending migrations automatically on startup.

## Structure

```
database/
  entity/       # SeaORM entity definitions (table structs, column enums)
    src/
      users.rs
      refresh_tokens.rs
  migration/    # SeaORM migration runner
    src/
      lib.rs    # Migration registry
      m20260307_175048_create_users.rs
      m20260307_190000_create_refresh_tokens.rs
```
