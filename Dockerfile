FROM denoland/deno:bin-2.9.8 AS deno

FROM rust:1-slim-bookworm AS build

RUN apt-get update && apt-get install -y ca-certificates musl-tools pkg-config

RUN rustup target add "$(uname -m)-unknown-linux-musl"

COPY --from=deno /deno /usr/local/bin/deno

RUN cargo install sqlx-cli --version 0.9.0 --locked --no-default-features --features sqlite

WORKDIR /app
COPY . .

RUN CARGO_BUILD_TARGET="$(uname -m)-unknown-linux-musl" TARGET_CC=musl-gcc deno task build \
    && cp "target/$(uname -m)-unknown-linux-musl/release/bons" /bons \
    && strip /bons

# Prepare a writable directory for the non-root runtime user.
RUN mkdir /data

FROM scratch AS runtime

COPY --from=build /bons /usr/local/bin/bons

WORKDIR /data
ENV BONS_DATABASE=/data/bons.sqlite
VOLUME ["/data"]
EXPOSE 2667

ENTRYPOINT ["/usr/local/bin/bons"]
CMD ["serve", "--host", "0.0.0.0"]
