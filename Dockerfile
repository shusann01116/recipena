FROM rust:1@sha256:25038aa450210c53cf05dbf7b256e1df1ee650a58bb46cbc7d6fa79c1d98d083 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:c53c9416a1acdbfd6e09abba720442444a3d1a6338b8db850e5e198b59af5570

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
