FROM rust:latest as builder
COPY . .
RUN cargo build --release
WORKDIR /target/release
FROM debian:stable
RUN apt-get update
RUN apt-get -y install libssl-dev libpq-dev
COPY --from=builder /target/release/psico_server .
COPY --from=builder /static/ ./static
CMD ["./psico_server"]