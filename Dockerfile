FROM rust:1@sha256:200279345fe5e243342afaf2b7af170a6de22faf86b486a561c97fa35f9920d5 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:0000f9dc0290f8eaf0ecceafbc35e803649087ea7879570fbc78372df7ac649b

COPY --from=builder /app/target/release/recipena /recipena

ENTRYPOINT ["/recipena"]
