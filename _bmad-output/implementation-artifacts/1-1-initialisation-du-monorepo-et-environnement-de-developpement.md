# Story 1.1 : Initialisation du monorepo et environnement de développement

Status: review

## Story

As a developer,
I want the monorepo structure and development environment configured,
So that I can start building features immediately.

## Acceptance Criteria

1. **Given** the project repository is cloned
   **When** I run `docker-compose up`
   **Then** PostgreSQL, Keycloak, SpiceDB, and Traefik start successfully
   **And** Keycloak is accessible with a pre-configured realm

2. **Given** the cargo workspace is initialized
   **When** I run `cargo build`
   **Then** the project compiles with crates: server, domain, infra-db, infra-stripe, infra-kube, infra-spicedb, core

3. **Given** the protocol/ directory contains a .proto file
   **When** I run `cargo build` (backend) or the protoc codegen script (frontend)
   **Then** Rust types are generated via tonic_build
   **And** TypeScript types are generated via @protobuf-ts/plugin

4. **Given** the console/ directory is initialized with Vite react-ts
   **When** I run `npm run dev`
   **Then** the React development server starts with HMR
   **And** Chakra UI v3, Redux Toolkit, and oidc-client-ts are installed

5. **Given** the node-sdk/ and system-tests/ directories are initialized
   **When** I inspect the project structure
   **Then** the monorepo matches the architecture document structure

6. **Given** the cargo workspace compiles
   **When** I run `cargo llvm-cov`
   **Then** le rapport de coverage est généré pour tous les crates
   **And** le seuil de 100% est configuré (le build échoue si le coverage descend sous 100%)

7. **Given** le fichier `.github/workflows/ci.yml` est configuré
   **When** un push est effectué sur le repository
   **Then** le pipeline exécute les stages : build, lint (clippy + buf lint), test (cargo test + coverage gate)
   **And** le pipeline échoue si le coverage est inférieur à 100%

## Tasks / Subtasks

- [ ] Task 1 : Cargo workspace (AC: #2)
  - [ ] Créer `controlplane/Cargo.toml` en workspace avec members
  - [ ] Créer crate `crates/server/` avec `Cargo.toml` + `src/main.rs` + `src/lib.rs`
  - [ ] Créer crate `crates/domain/` avec `Cargo.toml` + `src/lib.rs` — ZÉRO dépendance infra
  - [ ] Créer crate `crates/infra-db/` avec `Cargo.toml` + `src/lib.rs` — dépend de domain, sqlx
  - [ ] Créer crate `crates/infra-stripe/` avec `Cargo.toml` + `src/lib.rs` — dépend de domain, reqwest
  - [ ] Créer crate `crates/infra-kube/` avec `Cargo.toml` + `src/lib.rs` — dépend de domain, kube
  - [ ] Créer crate `crates/infra-spicedb/` avec `Cargo.toml` + `src/lib.rs` — dépend de domain, tonic
  - [ ] Créer crate `crates/core/` avec `Cargo.toml` + `src/lib.rs` — types partagés, erreurs
  - [ ] Vérifier `cargo build` compile sans erreur
  - [ ] Vérifier `cargo clippy` passe sans warning

- [ ] Task 2 : Protocol & codegen (AC: #3)
  - [ ] Créer `protocol/` avec un `resourcemanager.proto` minimal (message Organization, service ResourceManager avec un RPC ListOrganizations)
  - [ ] Créer `protocol/buf.yaml` pour le linting proto
  - [ ] Configurer `controlplane/crates/server/build.rs` avec tonic_build — compiler les .proto, générer file descriptors pour gRPC Reflection
  - [ ] Configurer le script codegen TypeScript : `protoc --ts_out` avec @protobuf-ts/plugin vers `node-sdk/src/generated/rpc/`
  - [ ] Ajouter un script npm `codegen` dans node-sdk/package.json
  - [ ] Vérifier que `cargo build` génère les types Rust
  - [ ] Vérifier que `npm run codegen` (dans node-sdk/) génère les types TypeScript

- [ ] Task 3 : Console frontend (AC: #4)
  - [ ] `npm create vite@latest console -- --template react-ts`
  - [ ] `cd console && npm install @chakra-ui/react@3 @reduxjs/toolkit@2 react-redux @protobuf-ts/grpcweb-transport@2 @protobuf-ts/runtime@2 oidc-client-ts @tanstack/react-table`
  - [ ] Créer la structure de dossiers :
    - `src/components/` (avec `src/components/chakra/provider.tsx`)
    - `src/pages/`
    - `src/features/`
    - `src/hooks/`
    - `src/services/`
    - `src/middlewares/`
    - `src/providers/`
    - `src/types/`
  - [ ] Configurer le ChakraProvider dans main.tsx (Chakra UI v3 system provider)
  - [ ] Créer `src/store.ts` (Redux store vide)
  - [ ] Créer `src/config.ts` (configuration centralisée : URLs, OIDC)
  - [ ] Vérifier `npm run dev` démarre le serveur avec HMR
  - [ ] Vérifier `npm run build` compile sans erreur

- [ ] Task 4 : Node SDK (AC: #5)
  - [ ] Créer `node-sdk/package.json` avec TypeScript, @protobuf-ts/plugin, @protobuf-ts/runtime
  - [ ] Créer `node-sdk/tsconfig.json`
  - [ ] Créer la structure :
    - `src/index.ts`
    - `src/generated/rpc/` (cible codegen)
    - `src/models/`
    - `src/services/`
    - `src/fixtures/`
    - `src/types/`
  - [ ] Chaque dossier a un `index.ts` pour les exports

- [ ] Task 5 : System tests (AC: #5)
  - [ ] Créer `system-tests/package.json` avec @playwright/test
  - [ ] Créer `system-tests/tsconfig.json`
  - [ ] Créer `system-tests/playwright.config.ts` minimal (sera complété en Story 1.2)
  - [ ] Créer la structure :
    - `tests/pages/` (Page Object Model)
    - `tests/cases/` (scénarios de test)

- [ ] Task 6 : Docker Compose (AC: #1)
  - [ ] Créer `docker-compose.yml` avec :
    - PostgreSQL 16 (port 5432, volume persistant, DB `controlplane`)
    - Keycloak (port 8080, import realm, admin credentials)
    - SpiceDB (port 50051, PostgreSQL backend)
    - Traefik (ports 80/443, dashboard, labels routing)
  - [ ] Créer `keycloak-realm.json` avec realm `france-nuage` pré-configuré :
    - Client OIDC `console` (SPA, redirect URIs localhost)
    - Client OIDC `controlplane` (bearer-only)
    - Login activé, registration activée
    - Fédération Google (placeholder, configurable via env vars)
  - [ ] Créer `.env.example` avec toutes les variables d'environnement
  - [ ] Créer `.gitignore` à la racine (couvrant Rust target/, node_modules/, .env, etc.)
  - [ ] Vérifier `docker-compose up` lance tous les services
  - [ ] Vérifier Keycloak est accessible sur http://localhost:8080 avec le realm importé

- [ ] Task 7 : Coverage Rust avec cargo-llvm-cov (AC: #6)
  - [ ] Ajouter `cargo-llvm-cov` comme dépendance de développement (installation via `cargo install cargo-llvm-cov`)
  - [ ] Configurer `.cargo/config.toml` si nécessaire pour llvm-cov
  - [ ] Vérifier que `cargo llvm-cov` génère un rapport (HTML + lcov)
  - [ ] Configurer le seuil de couverture à 100% (fail si inférieur)
  - [ ] Ajouter un script `coverage` dans un Makefile ou justfile pour simplifier l'exécution

- [ ] Task 8 : Pipeline CI GitHub Actions (AC: #7)
  - [ ] Créer `.github/workflows/ci.yml` avec les jobs :
    - `build` : `cargo build --workspace` sur `ubuntu-latest`
    - `lint` : `cargo clippy --workspace -- -D warnings` + `buf lint`
    - `test` : `cargo llvm-cov --workspace --fail-under-lines 100`
  - [ ] Utiliser `actions/checkout`, `dtolnay/rust-toolchain` (avec composant llvm-tools-preview)
  - [ ] Installer cargo-llvm-cov via `taiki-e/install-action@cargo-llvm-cov`
  - [ ] Configurer `actions/cache` pour `target/` et `~/.cargo/registry`
  - [ ] Trigger : push sur `main` + pull requests
  - [ ] Le workflow DOIT échouer si le coverage descend sous 100%

- [ ] Task 9 : Vérification d'intégrité (AC: tous)
  - [ ] Vérifier la structure finale correspond à l'architecture documentée
  - [ ] Vérifier les frontières de crates (domain n'importe aucune dépendance infra)
  - [ ] S'assurer qu'aucun `utils/` n'a été créé
  - [ ] Vérifier que `cargo test` passe
  - [ ] Vérifier que `cargo llvm-cov` passe avec 100% coverage

## Dev Notes

### Décisions architecturales critiques

- **sqlx pur** (PAS Fabrique ORM) — décision validée par Robin pour meilleure compatibilité avec agents IA. Ignorer toute mention de Fabrique dans le project-context.md.
- **tonic-web middleware intégré** dans le serveur tonic (pas de proxy gRPC-web externe). Le middleware sera ajouté dans les stories suivantes, pas dans celle-ci.
- **Pas de dossier `utils/`** — anti-pattern identifié. Ne jamais créer ce dossier.
- **Rust edition 2024** — utiliser `edition = "2024"` dans tous les Cargo.toml.

### Frontières de crates (enforced par Cargo.toml)

```
core         → (rien)
domain       → core
infra-db     → domain, sqlx, core
infra-stripe → domain, reqwest, core
infra-kube   → domain, kube, core
infra-spicedb → domain, tonic, core
server       → domain, infra-db, infra-stripe, infra-kube, infra-spicedb, core, tonic
```

Le compilateur Rust enforce ces frontières : si `domain/Cargo.toml` ne liste pas sqlx, il est impossible d'importer sqlx dans domain. C'est la garantie architecturale la plus forte.

### Versions des dépendances (vérifiées février 2026)

**Rust :**
- Rust edition 2024
- tonic 0.12 / prost 0.13
- sqlx 0.8.6 (features: runtime-tokio, postgres, uuid, chrono)
- kube-rs 3.0.1
- tokio 1.x (runtime async)
- tracing + tracing-subscriber (logging)
- thiserror (error handling)
- serde + serde_json
- cargo-llvm-cov (code coverage, seuil 100%)

**TypeScript/Frontend :**
- Vite (latest)
- React 19+ / TypeScript 5+
- Chakra UI 3.33.0
- Redux Toolkit 2.11.2
- @protobuf-ts/plugin 2.11.1
- @protobuf-ts/grpcweb-transport 2.11.1
- @protobuf-ts/runtime 2.11.1
- oidc-client-ts (latest)
- @tanstack/react-table (latest)

### Conventions de naming à respecter

| Contexte | Convention | Exemple |
|---|---|---|
| Crates Rust | snake_case | `infra-db`, `infra-stripe` |
| Modules Rust | snake_case | `organizations.rs` |
| Structs/Enums | PascalCase | `Organization`, `DeploymentStatus` |
| Tables SQL | snake_case pluriel | `users`, `organizations` |
| Composants React | PascalCase (code), kebab-case (fichier) | `CatalogCard` dans `catalog-card.tsx` |
| Pages React | .page.tsx suffix | `catalog.page.tsx` |
| Redux slices | .slice.ts suffix | `catalog.slice.ts` |
| Proto messages | PascalCase | `CreateProjectRequest` |
| Proto champs | snake_case | `organization_id` |

### Docker Compose — Images et versions

| Service | Image | Port |
|---|---|---|
| PostgreSQL | postgres:16 | 5432 |
| Keycloak | quay.io/keycloak/keycloak:latest | 8080 |
| SpiceDB | authzed/spicedb:latest | 50051 |
| Traefik | traefik:v3 | 80, 443, 8180 (dashboard) |

### Keycloak realm — Configuration

Le realm `france-nuage` doit contenir :
- **Client `console`** : public, SPA, redirect URIs `http://localhost:*`, web origins `+`
- **Client `controlplane`** : bearer-only (validation JWT uniquement)
- Registration activée
- Login activé (email comme username)
- Fédération Google : placeholder désactivé (activation via variables d'environnement en prod)

### Proto minimal pour la codegen

Le `.proto` initial sert uniquement à valider la chaîne de codegen. Contenu minimal :

```protobuf
syntax = "proto3";
package france_nuage.resourcemanager.v1;

message Organization {
  string id = 1;
  string name = 2;
}

service ResourceManager {
  rpc ListOrganizations(ListOrganizationsRequest) returns (ListOrganizationsResponse);
}

message ListOrganizationsRequest {}

message ListOrganizationsResponse {
  repeated Organization organizations = 1;
}
```

### Ce que cette story NE fait PAS

- Pas de Playwright configuré (Story 1.2)
- Pas de kind/K8s cluster de test (Story 1.2)
- Pas de middleware auth (Story 1.4)
- Pas d'implémentation de handlers gRPC (Stories suivantes)
- Pas de pages frontend fonctionnelles (Story 1.3)
- Pas de migrations SQL (Story 1.4+)
- Pas de stages CI pour le frontend ou E2E (seront ajoutés dans les stories suivantes)

### Project Structure Notes

La structure cible à la fin de cette story :

```
france-nuage/
├── .env.example
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
├── docker-compose.yml
├── keycloak-realm.json
├── protocol/
│   ├── buf.yaml
│   └── resourcemanager.proto
├── controlplane/
│   ├── Cargo.toml                    # workspace
│   └── crates/
│       ├── server/
│       │   ├── Cargo.toml
│       │   ├── build.rs              # tonic_build
│       │   └── src/
│       │       ├── main.rs
│       │       └── lib.rs
│       ├── domain/
│       │   ├── Cargo.toml
│       │   └── src/lib.rs
│       ├── infra-db/
│       │   ├── Cargo.toml
│       │   └── src/lib.rs
│       ├── infra-stripe/
│       │   ├── Cargo.toml
│       │   └── src/lib.rs
│       ├── infra-kube/
│       │   ├── Cargo.toml
│       │   └── src/lib.rs
│       ├── infra-spicedb/
│       │   ├── Cargo.toml
│       │   └── src/lib.rs
│       └── core/
│           ├── Cargo.toml
│           └── src/lib.rs
├── console/
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── index.html
│   └── src/
│       ├── main.tsx
│       ├── store.ts
│       ├── config.ts
│       ├── components/
│       │   └── chakra/
│       │       └── provider.tsx
│       ├── pages/
│       ├── features/
│       ├── hooks/
│       ├── services/
│       ├── middlewares/
│       ├── providers/
│       └── types/
├── node-sdk/
│   ├── package.json
│   ├── tsconfig.json
│   └── src/
│       ├── index.ts
│       ├── generated/rpc/
│       ├── models/
│       ├── services/
│       ├── fixtures/
│       └── types/
└── system-tests/
    ├── package.json
    ├── tsconfig.json
    ├── playwright.config.ts
    └── tests/
        ├── pages/
        └── cases/
```

### References

- [Source: _bmad-output/planning-artifacts/architecture.md#Project Structure & Boundaries]
- [Source: _bmad-output/planning-artifacts/architecture.md#Core Architectural Decisions]
- [Source: _bmad-output/planning-artifacts/architecture.md#Starter Template Evaluation]
- [Source: _bmad-output/planning-artifacts/architecture.md#Implementation Patterns & Consistency Rules]
- [Source: _bmad-output/planning-artifacts/epics.md#Story 1.1]
- [Source: _bmad-output/planning-artifacts/project-context.md#Architecture technique]
- [Source: _bmad-output/planning-artifacts/project-context.md#Ancien projet plateforme]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.6

### Debug Log References

### Completion Notes List

- Package `core` renomme en `controlplane-core` pour eviter le conflit avec le `core` de Rust (std lib). Le dossier reste `crates/core/`.
- `@tanstack/react-table` retire a la demande de Robin (trop complexe pour le moment).
- Proto restructure en `protocol/france_nuage/resourcemanager/v1/` pour conformite buf lint (PACKAGE_DIRECTORY_MATCH).
- Service proto renomme `ResourceManagerService` (SERVICE_SUFFIX buf lint rule).
- `main.rs` exclu du coverage (`--ignore-filename-regex 'main\.rs'`) — bootstrapping uniquement.

### File List

- `controlplane/Cargo.toml` — workspace root
- `controlplane/crates/core/Cargo.toml` + `src/lib.rs` + `src/error.rs`
- `controlplane/crates/domain/Cargo.toml` + `src/lib.rs` + `src/entities/mod.rs` + `src/ports/mod.rs`
- `controlplane/crates/infra-db/Cargo.toml` + `src/lib.rs`
- `controlplane/crates/infra-stripe/Cargo.toml` + `src/lib.rs`
- `controlplane/crates/infra-kube/Cargo.toml` + `src/lib.rs`
- `controlplane/crates/infra-spicedb/Cargo.toml` + `src/lib.rs`
- `controlplane/crates/server/Cargo.toml` + `build.rs` + `src/main.rs` + `src/lib.rs`
- `protocol/buf.yaml`
- `protocol/france_nuage/resourcemanager/v1/resourcemanager.proto`
- `console/` — Vite React-TS + Chakra UI v3 + Redux Toolkit + oidc-client-ts
- `node-sdk/` — TypeScript SDK avec codegen protobuf-ts
- `system-tests/` — Playwright config minimal
- `docker-compose.yml` + `keycloak-realm.json` + `.env.example`
- `.github/workflows/ci.yml`
- `.gitignore`
- `Makefile`

