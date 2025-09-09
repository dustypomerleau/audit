from rustlang/rust:nightly-alpine as builder

run apk update && apk add --no-cache bash curl libc-dev binaryen
run cargo install cargo-binstall
run curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
run rustup target add wasm32-unknown-unknown

workdir /work
copy . .
run cargo leptos build --release -vv

from rustlang/rust:nightly-alpine as runner

workdir /app
copy --from=builder /work/target/release/audit /app/
copy --from=builder /work/target/site /app/site
copy --from=builder /work/Cargo.toml /app/

env RUST_LOG="info"
env LEPTOS_SITE_ADDR="0.0.0.0:8080"
env LEPTOS_SITE_ROOT=./site
expose 8080

cmd ["/app/audit"]

