FROM rust:1@sha256:c234989a3d67801c51e6ce3149b1c17f61a316baa5fd1a5d5c83d5fe7e1817f1 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:72344f7f909a8bf003c67f55687e6d51a441b49661af8f660aa7b285f00e57df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
