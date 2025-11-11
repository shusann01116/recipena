FROM rust:1@sha256:cd34b27bc6df5450e4952075dc6bd3881a1aeb8f5d0478cd75c2160ca47e2182 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0000f9dc0290f8eaf0ecceafbc35e803649087ea7879570fbc78372df7ac649b

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
