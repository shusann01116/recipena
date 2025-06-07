#!/bin/sh
# Script to copy SSL libraries for the correct architecture

TARGETARCH=${TARGETARCH:-amd64}

# Map Docker platform architecture to library directory names
case "$TARGETARCH" in
    "amd64")
        ARCH_DIR="x86_64-linux-gnu"
        ;;
    "arm64")
        ARCH_DIR="aarch64-linux-gnu"
        ;;
    *)
        echo "Unsupported architecture: $TARGETARCH"
        exit 1
        ;;
esac

echo "Copying SSL libraries for architecture: $TARGETARCH ($ARCH_DIR)"

# Create target directory
mkdir -p "/usr/lib/$ARCH_DIR"

# Copy SSL libraries from builder stage
cp "/builder-libs/$ARCH_DIR/libssl.so.3" "/usr/lib/$ARCH_DIR/"
cp "/builder-libs/$ARCH_DIR/libcrypto.so.3" "/usr/lib/$ARCH_DIR/"

echo "SSL libraries copied successfully"