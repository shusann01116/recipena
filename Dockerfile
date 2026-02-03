FROM rust:1@sha256:e35d0f677e0e0be6f4b4f93bf16e6f93ab4f427dc440a0ef12511026f8b7d7e3 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:72344f7f909a8bf003c67f55687e6d51a441b49661af8f660aa7b285f00e57df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
