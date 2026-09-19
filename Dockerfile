FROM rust:1-bookworm AS builder

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo leptos build --release --precompress --split -vv

RUN cargo run -p xtask --release -- preprocess

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/markusunkel-com /app/markusunkel-com
COPY --from=builder /app/target/release/hash.txt /app/hash.txt
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/target/site-assets /app/target/site-assets
COPY --from=builder /app/content /app/content
COPY --from=builder /app/crates/site/Cargo.toml /app/Cargo.toml

ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="target/site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_HASH_FILES="true"

EXPOSE 8080
CMD ["/app/markusunkel-com"]
