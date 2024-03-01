FROM rust:slim-buster as builder

WORKDIR /usr/src

RUN USER=root cargo new mhfrontier-cq-tool-api

WORKDIR /usr/src/mhfrontier-cq-tool-api

COPY Cargo.toml Cargo.lock /usr/src/mhfrontier-cq-tool-api/

RUN cargo build --release

COPY src /usr/src/mhfrontier-cq-tool-api/src/

RUN touch /usr/src/mhfrontier-cq-tool-api/src/main.rs

RUN cargo build --release

FROM debian:buster-slim

EXPOSE 8080

COPY --from=builder /usr/src/mhfrontier-cq-tool-api/target/release/mhfrontier-cq-tool-api /usr/local/bin/mhfrontier-cq-tool-api

WORKDIR /usr/local/bin/

CMD ["./mhfrontier-cq-tool-api"]
