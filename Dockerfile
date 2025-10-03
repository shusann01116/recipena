FROM rust:1@sha256:512d81e488da6ba3e88637340fa84decd29293185d48e4708468e1b1d3bb20a9 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:14f6999db515330e5d00537bd457289a8968b6456e9197c7a28101ee63a7522f

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
