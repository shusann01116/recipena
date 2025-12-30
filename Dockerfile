FROM rust:1@sha256:33f31e4450323fef5c784d433f136e70d16f4865047d7b1a969b67696a274ebe AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0c8eac8ea42a167255d03c3ba6dfad2989c15427ed93d16c53ef9706ea4691df

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
