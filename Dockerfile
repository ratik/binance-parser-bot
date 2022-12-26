FROM rust:1.66.0 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . && cargo build --release
FROM debian:bullseye AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/src/app/target/release/binance_p2p_parser /usr/bin/binance_p2p_parser
ENTRYPOINT ["/usr/bin/binance_p2p_parser"]
