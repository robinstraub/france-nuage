# France-nuage

## Getting Started

### 1. Generate TLS certificates

```bash
./generate-certs.sh
```

### 2. Trust the CA certificate

**macOS:**

```bash
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain certs/ca.pem
```

**Linux (Ubuntu/Debian):**

```bash
sudo cp certs/ca.pem /usr/local/share/ca-certificates/france-nuage-dev-ca.crt
sudo update-ca-certificates
```

Restart your browser after trusting the CA.

### 3. Start the stack

```bash
docker compose up
```

### 4. Access the app

Open https://console.localhost in your browser.

| Service | URL |
|---|---|
| Console | https://console.localhost |
| Keycloak | https://keycloak.localhost |
| Controlplane (gRPC) | https://controlplane.localhost |
| Traefik dashboard | http://localhost:8180 |
