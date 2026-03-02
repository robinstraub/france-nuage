#!/bin/bash
set -euo pipefail

# Certificate generation script for Docker Compose development
# Usage: ./generate-certs.sh [--force]

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
CERTS_DIR="$PROJECT_DIR/certs"
TRAEFIK_DIR="$PROJECT_DIR/traefik"
FORCE_REGENERATE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
  --force | -f)
    FORCE_REGENERATE=true
    shift
    ;;
  --help | -h)
    echo "Usage: $0 [--force]"
    echo "Generate SSL certificates for Docker Compose development"
    echo ""
    echo "Options:"
    echo "  --force, -f    Force regeneration even if certificates exist"
    echo "  --help, -h     Show this help message"
    exit 0
    ;;
  *)
    echo -e "${RED}Error: Unknown option $1${NC}"
    echo "Use --help for usage information"
    exit 1
    ;;
  esac
done

# Logging functions
log_info() {
  echo -e "${BLUE}[info]${NC} $1"
}

log_success() {
  echo -e "${GREEN}[ok]${NC} $1"
}

log_warning() {
  echo -e "${YELLOW}[warn]${NC} $1"
}

log_error() {
  echo -e "${RED}[error]${NC} $1"
}

# Check dependencies
check_dependencies() {
  log_info "Checking dependencies..."

  if ! command -v openssl >/dev/null 2>&1; then
    log_error "Missing required dependency: openssl"
    echo ""
    echo "Please install openssl:"
    echo "  Ubuntu/Debian: sudo apt-get install openssl"
    echo "  macOS: brew install openssl"
    exit 1
  fi

  log_success "All dependencies satisfied"
}

# Check if certificates already exist
check_existing_certs() {
  if [ "$FORCE_REGENERATE" = true ]; then
    log_warning "Force regeneration requested, will overwrite existing certificates"
    return 0
  fi

  if [ -f "$CERTS_DIR/ca.pem" ] && [ -f "$CERTS_DIR/server.pem" ] && [ -f "$CERTS_DIR/server-key.pem" ]; then
    log_warning "Certificates already exist in $CERTS_DIR"
    echo ""
    echo "Use --force to regenerate or remove the existing certificates manually."
    exit 0
  fi
}

# Generate CA certificate
generate_ca() {
  log_info "Generating Certificate Authority (CA)..."

  mkdir -p "$CERTS_DIR" "$TRAEFIK_DIR"

  openssl genrsa -out "$CERTS_DIR/ca-key.pem" 4096

  openssl req -new -x509 -days 3650 -key "$CERTS_DIR/ca-key.pem" -out "$CERTS_DIR/ca.pem" \
    -subj "/C=FR/ST=Development/L=Docker/O=France-nuage/OU=Dev/CN=France-nuage Dev CA"

  log_success "CA certificate generated"
}

# Generate server certificate
generate_server_cert() {
  log_info "Generating server certificate..."

  openssl genrsa -out "$CERTS_DIR/server-key.pem" 4096

  cat >"$CERTS_DIR/server-ext.cnf" <<EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
prompt = no

[req_distinguished_name]
C = FR
ST = Development
L = Docker
O = France-nuage
OU = Dev
CN = localhost

[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
DNS.2 = *.localhost
DNS.3 = console.localhost
DNS.4 = keycloak.localhost
DNS.5 = controlplane.localhost
DNS.6 = host.docker.internal
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

  openssl req -new -key "$CERTS_DIR/server-key.pem" -out "$CERTS_DIR/server.csr" \
    -config "$CERTS_DIR/server-ext.cnf"

  openssl x509 -req -days 365 -in "$CERTS_DIR/server.csr" \
    -CA "$CERTS_DIR/ca.pem" -CAkey "$CERTS_DIR/ca-key.pem" \
    -out "$CERTS_DIR/server.pem" \
    -extensions v3_req -extfile "$CERTS_DIR/server-ext.cnf" \
    -CAcreateserial

  rm "$CERTS_DIR/server.csr" "$CERTS_DIR/server-ext.cnf"

  log_success "Server certificate generated"
}

# Generate Traefik dynamic configuration
generate_traefik_config() {
  log_info "Generating Traefik dynamic configuration..."

  cat >"$TRAEFIK_DIR/dynamic.yml" <<EOF
tls:
  certificates:
    - certFile: /etc/traefik/certs/server.pem
      keyFile: /etc/traefik/certs/server-key.pem
      stores:
        - default
  stores:
    default:
      defaultCertificate:
        certFile: /etc/traefik/certs/server.pem
        keyFile: /etc/traefik/certs/server-key.pem
EOF

  log_success "Traefik configuration generated"
}

# Set appropriate permissions
set_permissions() {
  log_info "Setting certificate permissions..."

  chmod 600 "$CERTS_DIR/ca-key.pem" "$CERTS_DIR/server-key.pem"
  chmod 644 "$CERTS_DIR/ca.pem" "$CERTS_DIR/server.pem"

  if [ -f "$CERTS_DIR/ca.srl" ]; then
    chmod 644 "$CERTS_DIR/ca.srl"
  fi

  log_success "Permissions set correctly"
}

# Show system trust instructions
show_trust_instructions() {
  log_info "To trust the CA certificate on your system:"
  echo ""

  case "$(uname -s)" in
  Linux*)
    echo "Linux (Ubuntu/Debian):"
    echo "  sudo cp $CERTS_DIR/ca.pem /usr/local/share/ca-certificates/france-nuage-dev-ca.crt"
    echo "  sudo update-ca-certificates"
    ;;
  Darwin*)
    echo "macOS:"
    echo "  sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain $CERTS_DIR/ca.pem"
    ;;
  esac

  echo ""
  log_warning "You'll need to restart your browser after trusting the CA certificate"
}

# Main execution
main() {
  echo -e "${BLUE}France-nuage SSL Certificate Generator${NC}"
  echo "========================================"
  echo ""

  check_dependencies
  check_existing_certs
  generate_ca
  generate_server_cert
  generate_traefik_config
  set_permissions

  echo ""
  log_success "Certificates generated successfully!"
  echo ""

  show_trust_instructions

  echo ""
  echo "Next steps:"
  echo "  1. Trust the CA certificate using the instructions above"
  echo "  2. Start Docker Compose: docker compose up"
  echo "  3. Open https://console.localhost"
}

main "$@"
