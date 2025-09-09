FROM rust:1@sha256:1ca9500fa119fe67cc67de86fe0ce8c77d747bfb541d227cd6eca463d29cc454 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:620d8b11ae800f0dbd7995f89ddc5344ad603269ea98770588b1b07a4a0a6872

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
