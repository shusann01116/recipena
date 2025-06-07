ARG TARGETARCH
ARG TARGETOS

FROM rust:1@sha256:25038aa450210c53cf05dbf7b256e1df1ee650a58bb46cbc7d6fa79c1d98d083 AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM eclipse-temurin:17-jre AS downloader

RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    && rm -rf /var/lib/apt/lists/*

RUN wget -O tabula.zip https://github.com/tabulapdf/tabula/releases/download/v1.2.1/tabula-jar-1.2.1.zip && \
    unzip tabula.zip && \
    mv tabula/tabula.jar /tabula.jar && \
    rm -rf tabula.zip tabula/

# Intermediate stage to handle SSL library copying with shell access
FROM debian:12-slim AS ssl-copier

ARG TARGETARCH

# Copy SSL libraries from builder
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl.so.3 /builder-libs/x86_64-linux-gnu/libssl.so.3
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypto.so.3 /builder-libs/x86_64-linux-gnu/libcrypto.so.3
COPY --from=builder /usr/lib/aarch64-linux-gnu/libssl.so.3 /builder-libs/aarch64-linux-gnu/libssl.so.3
COPY --from=builder /usr/lib/aarch64-linux-gnu/libcrypto.so.3 /builder-libs/aarch64-linux-gnu/libcrypto.so.3

# Copy and run the SSL library setup script
COPY copy-ssl-libs.sh /tmp/copy-ssl-libs.sh
RUN chmod +x /tmp/copy-ssl-libs.sh && /tmp/copy-ssl-libs.sh

FROM gcr.io/distroless/java21-debian12

COPY --from=builder /app/target/release/recipena /recipena
COPY --from=downloader /tabula.jar /tabula.jar
COPY --from=ssl-copier /usr/lib /usr/lib

ENTRYPOINT ["/recipena"]
