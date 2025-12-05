FROM rust:1.91 as chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY libs libs
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 as runtime

WORKDIR /app

COPY --from=builder /app/target/release/* .

USER nonroot:nonroot

EXPOSE 5001
CMD ["./mhfrontier-cq-tool-api"]
