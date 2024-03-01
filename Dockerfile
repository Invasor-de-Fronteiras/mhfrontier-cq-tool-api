FROM rust:slim-buster as builder

WORKDIR /app

ARG APP_NAME=mhfrontier-cq-tool-api

COPY . .

RUN cargo build --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:buster-slim as runtime

COPY --from=builder /bin/server /bin/
EXPOSE 8080

WORKDIR /usr

CMD ["/bin/server"]

