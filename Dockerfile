FROM rust:1.76-bookworm AS builder

WORKDIR /frosthaven

COPY ./src ./src
COPY ./Cargo.toml .

RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /frosthaven

COPY --from=builder /usr/local/cargo/bin/frosthaven ./frosthaven

COPY ./static ./static
COPY ./tera ./tera


EXPOSE 8080
ENTRYPOINT ./frosthaven