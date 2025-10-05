FROM rust:1@sha256:976303ceda00c5f21d6fe97500927285c7e0f6a2e8df71ae18a6c8e9b37550a1 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:14f6999db515330e5d00537bd457289a8968b6456e9197c7a28101ee63a7522f

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
