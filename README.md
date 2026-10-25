# Bons

## Server configuration

Configure the container or process through environment variables; no `.env` file
is loaded.

| Variable        | Required | Description                                                                                               |
| --------------- | -------- | --------------------------------------------------------------------------------------------------------- |
| `BONS_CURRENCY` | Yes      | Currency code used by the server, such as `EUR` or `USD`. Server startup fails if it is missing or blank. |

For local development, `.cargo/config.toml` supplies `BONS_CURRENCY=EUR` to
Cargo commands unless it is already set in the environment. This is not a
compiled-in default: running the binary directly requires the variable at
runtime.

Known currency codes are generated at build time from Deno's
`Intl.supportedValuesOf("currency")`; building requires `deno` on `PATH`.
Unknown currency values are accepted and preserved, but produce a warning on
stderr at server startup. Missing or blank values remain errors.

### Currency API

`GET /api/currency` returns the configured currency as plain text
(`text/plain; charset=utf-8`), for example `EUR`. Authentication is not
required.

## Frontend data access

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

Stop the backend and remove the disposable `bons.sqlite` database, then run:

```sh
cargo run --example seed
deno task dev
```

The example creates a fresh `bons.sqlite` with an admin named `Development` and
six entries each for the current and previous UTC calendar months. Log in with
the following development-only access key (43 `A` characters):

```text
AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
```

This public key must never be used in production. The example refuses to
overwrite existing files. To create a separate database, use
`cargo run --example seed -- --database path/to/development.sqlite`.

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
