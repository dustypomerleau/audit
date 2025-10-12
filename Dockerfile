# see https://github.com/leptos-rs/leptos-website/blob/main/Dockerfile for reference

FROM rustlang/rust:nightly-alpine as builder

RUN apk update \
    && apk add --no-cache \
        bash \
        binaryen \
        chromium-chromedriver \
        curl \
        libc-dev \
        openssl-dev \
        openssl-libs-static \
        perl \
        pkgconfig

RUN cargo install cargo-binstall
RUN cargo binstall cargo-leptos
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .
RUN RUSTFLAGS="--cfg erase_components" cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app
COPY --from=builder /work/target/release/audit /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV LEPTOS_SITE_ADDR=0.0.0.0:$PORT
ENV LEPTOS_SITE_ROOT=site
ENV RUST_LOG=info
EXPOSE $PORT

CMD ["/app/audit"]
