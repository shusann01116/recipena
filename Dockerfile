FROM rust:1@sha256:e27f43cfd678d25d6c87f1a4775619bccc02505fe6809806f2d5831731b495d8 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0c8eac8ea42a167255d03c3ba6dfad2989c15427ed93d16c53ef9706ea4691df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
