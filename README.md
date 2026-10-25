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
