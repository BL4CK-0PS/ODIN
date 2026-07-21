#!/bin/bash
# =============================================================================
# Generate self-signed TLS certificates for nginx
# Usage: ./generate-certs.sh
# =============================================================================

set -euo pipefail

CERT_DIR="$(cd "$(dirname "$0")" && pwd)/certs"
DAYS=365

mkdir -p "$CERT_DIR"

if [ -f "$CERT_DIR/server.crt" ] && [ -f "$CERT_DIR/server.key" ]; then
    echo "Certificates already exist in $CERT_DIR. Remove them to regenerate."
    exit 0
fi

echo "[+] Generating self-signed TLS certificate..."
openssl req -x509 -nodes -days "$DAYS" \
    -newkey rsa:2048 \
    -keyout "$CERT_DIR/server.key" \
    -out "$CERT_DIR/server.crt" \
    -subj "/C=US/ST=Local/L=Local/O=ODIN/CN=localhost" \
    -addext "subjectAltName=DNS:localhost,DNS:odin-api,DNS:odin-web,IP:127.0.0.1"

chmod 600 "$CERT_DIR/server.key"
chmod 644 "$CERT_DIR/server.crt"

echo "[+] Certificates generated in $CERT_DIR"
echo "    server.crt (public key)"
echo "    server.key (private key)"
