# Bons

## Server configuration

Configure the container or process through environment variables; no `.env` file
is loaded.

| Variable        | Required | Description                                                                                                |
| --------------- | -------- | ---------------------------------------------------------------------------------------------------------- |
| `BONS_CURRENCY` | No       | Overrides the server's built-in `EUR` currency default, for example with `USD`. Blank values are rejected. |
| `BONS_DATABASE` | No       | Runtime SQLite database path. Defaults to `bons.sqlite`. Separate from SQLx's compile-time `DATABASE_URL`. |

The server defaults to `EUR`, including when running the binary directly.
Set `BONS_CURRENCY` at runtime to use a different currency.

Known currency codes are generated at build time from Deno's
`Intl.supportedValuesOf("currency")`; building requires `deno` on `PATH`.
Unknown currency values are accepted and preserved, but produce a warning on
stderr at server startup. Blank values remain errors.

### Currency API

`GET /api/currency` returns the configured currency as plain text
(`text/plain; charset=utf-8`), for example `EUR`. Authentication is not
required.

## Frontend data access

The backend serves the Vite output from `dist/` at `/`, with `index.html` as
the fallback for client-side routes. API routes remain separate; missing API
paths and missing assets return `404`.

Debug builds read `dist/` from disk. Release builds embed it into the executable,
so no separate frontend files are needed at runtime. Build the frontend before
compiling a release. `deno task build` builds the frontend followed by the release
backend with embedded assets. `deno task dev` uses the debug backend and the Vite
development server with its API proxy.

The backend binds to `127.0.0.1:2667` by default, and the development proxy uses
that same IPv4 address to avoid localhost IPv4/IPv6 fallback delays. Use
`serve --host <address>` to bind the backend to a different interface.

- `frontend/services/api.ts` owns HTTP requests and uses the generated types in
  `bindings/`. HTTP failures throw `ApiError` with a status code; network
  failures propagate unchanged.
- Composables in `frontend/composables/` own reactive state and expose it to Vue
  components. For example, `useCurrency.ts` loads currency through the API
  service and exposes the formatter used by templates.
- Components own presentation and UI interactions, not HTTP details. Login uses
  `useAuth.ts`; month pages load spending through `useMonthSpending.ts` and save
  entries through `useCreateSpending.ts`.

## Development data

Start development with:

```sh
deno task dev
```

Before starting the backend, this runs `db:prepare` to prepare SQLx's
compile-time database, then `db:seed` to recreate the disposable runtime
database at `target/dev.sqlite`. The seed and backend tasks both set
`BONS_DATABASE=target/dev.sqlite`. Every development restart deletes this
database and its SQLite sidecars via `db:clean`, then seeds fresh data. Stop the
running development backend before resetting it. The default `bons.sqlite` is
never touched by these tasks.

The example creates a new database with an admin named `Development` and 106
entries across the current and five previous UTC calendar months. Each month has
different purchases, amounts, and entry counts (between 7 and 34), with dates
distributed throughout the actual calendar month. The seed example fails without
modifying an existing database. Log in with the following development-only
access key:

```text
bons2667-all-your-receipts-are-belong-to-us
```

This public key must never be used in production. To seed without starting the
servers, run `deno task db:seed` (this also deletes the old development
database). To seed a separate new database without resetting the development
database, set `BONS_DATABASE` to its path and run `cargo run --example seed`
after `deno task db:prepare`. The example uses `BONS_DATABASE`, falling back to
`bons.sqlite` just like the backend.

Resetting development data also removes any development users, entries, and
sessions.

These tasks require Deno, Cargo, and `sqlx-cli`. The SQLx database and query
metadata are generated locally, not checked in. If running Cargo directly, run
`deno task db:prepare` first.

### Authentication

`POST /api/login` accepts `{ "key": "..." }` and sets a 30-day HttpOnly session
cookie. The browser manages that cookie; do not store access keys or session
tokens in local storage. API requests use same-origin URLs, proxied to the
backend by Vite in development.

`GET /api/currency` and `POST /api/login` are public. Reading monthly spending
and creating spending entries require a valid session (`401` otherwise).
Creating users also requires an admin (`403` otherwise).

`GET /api/me` returns the current user and requires a valid session. The
frontend checks it before each navigation: unauthenticated visitors go to
`/login`, and authenticated visitors to `/login` go to `/`. Authenticated
visitors can stay on their selected month page, including after a reload. There
is no logout endpoint yet.

Session cookies are Secure by default. For local HTTP development,
`.cargo/config.toml` sets `INSECURE_COOKIES=1` for Cargo commands; do not set it
in production.
