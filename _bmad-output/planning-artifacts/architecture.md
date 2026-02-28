---
stepsCompleted:
  - 1
  - 2
  - 3
  - 4
  - 5
  - 6
  - 7
  - 8
lastStep: 8
status: 'complete'
completedAt: '2026-02-27'
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/project-context.md
workflowType: 'architecture'
project_name: 'france-nuage'
user_name: 'Robin'
date: '2026-02-27'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**
33 FRs répartis en 8 domaines fonctionnels :
- **Authentification & Identité** (FR1-FR4) : inscription/connexion Keycloak OIDC, validation token, identification utilisateur/organisations
- **Gestion des organisations** (FR5-FR9) : CRUD organisations, hiérarchie parent-enfant interne, organisation racine France-nuage
- **Gestion des projets** (FR10-FR12) : création/consultation projets, contexte de déploiement des applications
- **Catalogue d'applications** (FR13-FR17) : consultation catalogue, métadonnées Helm (nom, description, variables, équivalent GAFAM), déclaration par DevOps
- **Provisionnement d'applications** (FR18-FR23) : formulaire dynamique depuis variables Helm, validation, déploiement K8s, feedback statut, URL d'accès
- **Gestion des applications déployées** (FR24-FR26) : liste avec statut, suppression avec confirmation, retrait Helm release
- **Billing & Facturation** (FR27-FR30) : customer Stripe automatique, subscription par org, items par app, consultation coûts
- **Autorisation ReBAC** (FR31-FR33) : vérification permissions SpiceDB, héritage via parent, accès complet par org au MVP

**Implications architecturales :** Le control plane est un orchestrateur entre 5 systèmes externes. Chaque FR de provisionnement ou billing implique des opérations coordonnées multi-systèmes. L'architecture doit isoler chaque intégration derrière une abstraction testable.

**Non-Functional Requirements:**
- **Sécurité :** authentification Keycloak obligatoire sur tous les endpoints, aucun secret dans les réponses/logs, seuls les identifiants de référence Stripe stockés, données en France (RGPD)
- **Qualité de code :** 100% code coverage Rust, modules < 500 lignes, .proto = source de vérité, fonctions publiques documentées, conventions pour agents IA
- **Fiabilité :** pas de ressources orphelines en cas d'échec (Helm release, subscription Stripe), opérations idempotentes ou compensables

**Scale & Complexity:**

- Domaine principal : Full-stack (API gRPC + SPA React)
- Niveau de complexité : Moyen (scope passe-plat, mais 5 intégrations externes et orchestration multi-systèmes)
- Composants architecturaux estimés : 8-10 modules backend, 6-8 domaines frontend

### Technical Constraints & Dependencies

**Contraintes fermes (décisions du project-context) :**
- Backend Rust (edition 2024) avec tonic/prost pour gRPC
- Frontend React + Chakra UI + Redux Toolkit + Vite
- Codegen TypeScript via @protobuf-ts/plugin
- ORM Fabrique (custom, SQL-first, type-safe, PostgreSQL)
- Keycloak en production (serveur existant)
- SpiceDB pour l'autorisation ReBAC
- Stripe pour le billing (forfait mensuel simple)
- K8s géré par DevOps, control plane applique les Helm charts
- Docker-compose pour le dev local (PG, Keycloak, SpiceDB, Traefik)

**Patterns à réutiliser de l'ancien projet (plateforme) :**
- gRPC-first API avec codegen bidirectionnelle
- Middleware gRPC avec injection Bearer token
- Structure Rust workspace multi-crates
- Schéma SpiceDB hiérarchique (org -> project -> ressources)
- gRPC Reflection pour le discovery
- buf.yaml pour le linting des .proto
- oidc-client-ts côté frontend
- CI/CD GitLab multi-stage

**Patterns à explorer du brouillon controlplane :**
- Intégration Stripe (customers, products, prices, subscriptions)
- Module CaaS (kube-rs)
- Fabrique ORM
- Architecture trait-based avec injection de dépendances (App<Billing, Caas>)

### Cross-Cutting Concerns Identified

1. **Authentification/Autorisation** : chaque requête API passe par validation token Keycloak + vérification permissions SpiceDB. Middleware gRPC dédié, intercepté avant le handler.
2. **Orchestration multi-systèmes** : le provisionnement et la suppression d'apps coordonnent K8s + Stripe + SpiceDB + PostgreSQL. Pattern de compensation nécessaire en cas d'échec partiel.
3. **Codegen & contrat API** : les .proto définissent le contrat, la modification d'un .proto déclenche la régénération Rust + TypeScript. Le pipeline de build doit être fiable.
4. **Observabilité** : logs structurés sans exposition de secrets, statut de provisionnement traçable.
5. **Testabilité** : 100% coverage impose des traits pour chaque intégration externe, permettant le mocking. Architecture hexagonale naturelle.

## Starter Template Evaluation

### Primary Technology Domain

Full-stack avec deux stacks indépendantes : API gRPC Rust (backend) + SPA React TypeScript (frontend). Pas de meta-framework, pas de SSR — architecture classique API + client séparé.

### Starter Options Considered

**Backend Rust :**
Aucun starter template pertinent. La combinaison tonic + prost + sqlx + Fabrique ORM + kube-rs est trop spécifique. L'ancien projet `plateforme/controlplane/` et le brouillon `controlplane/` fournissent les patterns de référence (workspace multi-crates, build.rs avec tonic_build, structure modulaire).

**Frontend React :**
Le starter Vite officiel (`react-ts`) est le seul candidat pertinent. Les starters communautaires ajoutent des opinions (Tailwind, routing, etc.) qui entrent en conflit avec la stack choisie (Chakra UI, Redux Toolkit).

### Selected Starters

**Backend : Cargo workspace from scratch**

**Rationale :** Stack trop spécifique pour un starter. Les patterns éprouvés de l'ancien projet servent de guide structurel.

**Initialization :**

```bash
cargo init controlplane --name controlplane
```

Puis conversion en workspace avec crates par domaine.

**Frontend : Vite react-ts**

**Rationale :** Template officiel minimal, compatible avec l'ajout de Chakra UI v3 et Redux Toolkit sans conflit.

**Initialization :**

```bash
npm create vite@latest console -- --template react-ts
cd console
npm install @chakra-ui/react@3 @reduxjs/toolkit@2 react-redux @protobuf-ts/grpcweb-transport@2 @protobuf-ts/runtime@2 oidc-client-ts @tanstack/react-table
```

### Technology Versions (vérifiées février 2026)

**Backend Rust :**
- Rust edition 2024
- tonic 0.12 / prost 0.13 (gRPC)
- sqlx 0.8.6 (PostgreSQL)
- kube-rs 3.0.1 (Kubernetes client)
- Fabrique (ORM custom, version interne)

**Frontend TypeScript :**
- Vite (latest)
- React 19+ / TypeScript 5+
- Chakra UI 3.33.0
- Redux Toolkit 2.11.2
- @protobuf-ts/plugin 2.11.1
- @protobuf-ts/grpcweb-transport 2.11.1
- oidc-client-ts (Keycloak OIDC)
- @tanstack/react-table (tableaux de données)

### Architectural Decisions Provided by Starters

**Vite react-ts fournit :**
- Configuration TypeScript (tsconfig strict)
- Hot Module Replacement (HMR)
- Build optimisé avec Rollup
- Structure de base (src/, public/, index.html)

**À configurer manuellement (pas fourni par le starter) :**
- Chakra UI provider et thème
- Redux store et slices
- Transport gRPC-web et codegen protobuf-ts
- OIDC provider (oidc-client-ts)
- Structure de dossiers par domaine fonctionnel
- ESLint / Prettier (conventions projet)

### Migration Notes

**Chakra UI v2 → v3 :** L'ancien projet utilisait Chakra UI v2. La v3 est une réécriture majeure. Le code UI de `plateforme/console/` ne peut pas être réutilisé tel quel — les patterns (layout, composants, theming) restent valides mais l'API change.

**Note :** L'initialisation des projets (cargo init + npm create vite) devrait être la première story d'implémentation.

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (bloquent l'implémentation) :**
1. Synchronisation utilisateurs : table `users` au premier login, pas de synchronizer
2. Orchestration multi-systèmes : K8s-first, compensation simple
3. Proxy gRPC-web : tonic-web (middleware intégré)
4. Structure monorepo : console, controlplane, node-sdk, protocol, system-tests

**Important Decisions (structurent l'architecture) :**
5. Migrations BDD : sqlx migrate
6. Routing frontend : React Router
7. Redux slices : par domaine fonctionnel
8. Logging backend : tracing (tokio)
9. Stratégie de test : unitaires Rust + E2E Playwright, pas de tests unitaires frontend

**Deferred Decisions (post-MVP) :**
- Caching (pas de besoin identifié au MVP)
- Rate limiting (pas de cible de performance MVP)
- API versioning (un seul client = la console)

### Data Architecture

**Base de données :** PostgreSQL via sqlx + Fabrique ORM

**Table `users` :**
- Créée au premier login depuis le token ID Keycloak
- Contient : ID Keycloak, email, nom
- Pas de synchronizer Keycloak → DB
- Keycloak = fédération d'identité uniquement

**Relations utilisateur ↔ organisation :**
- Table `organization_user` = source de vérité pour l'appartenance
- SpiceDB est reconstructible à 100% depuis la DB du control plane
- Les invitations créent la relation `organization_user` + la relation SpiceDB correspondante

**Migrations :** sqlx migrate (migrations SQL pures, vérification compile-time)

### Authentication & Security

**Authentification :** Keycloak OIDC
- Frontend : oidc-client-ts, token ID pour les infos utilisateur
- Backend : validation JWT sur chaque requête gRPC via middleware tonic
- Création user en base au premier login (depuis le token ID)

**Autorisation :** SpiceDB ReBAC
- Schéma hiérarchique : organization → project → application
- Vérification permissions avant chaque opération
- DB du control plane = source de vérité, SpiceDB = moteur de requête
- Reconstruction complète possible depuis la DB

**Customer Stripe :**
- Création du customer Stripe = flux séparé, déclenché par la console avant le premier déploiement
- Hors du tunnel de provisionnement d'app

### API & Communication Patterns

**Protocole :** gRPC (tonic/prost) avec gRPC-web via tonic-web middleware
- Pas de proxy externe (Envoy/Traefik) pour le gRPC-web
- Le control plane expose gRPC natif + gRPC-web dans le même binaire
- Traefik reste utilisé comme reverse proxy / TLS termination en dev local et prod

**Codegen :** .proto → Rust (tonic_build) + TypeScript (@protobuf-ts/plugin)
- Dossier `protocol/` partagé entre controlplane et node-sdk
- buf.yaml pour le linting des .proto
- gRPC Reflection pour le discovery d'API

**Gestion d'erreurs :** gRPC status codes standard
- Erreurs métier mappées sur les status codes gRPC appropriés
- Pas de secrets ni de données sensibles dans les messages d'erreur

### Orchestration Multi-Systèmes

**Provisionnement d'une application (ordre d'exécution) :**
1. **K8s** — Helm install (seul point réellement faillible)
   - Si échec → notification utilisateur, rien à compenser
2. **PostgreSQL** — créer l'enregistrement app (transaction SQL)
   - Si échec → Helm uninstall K8s
3. **SpiceDB** — créer les relations de permission
   - Si échec → rollback transaction PG + Helm uninstall
4. **Stripe** — ajouter l'item à la subscription (quasi infaillible)
   - Si échec → défaire relation SpiceDB + rollback PG + Helm uninstall

**Suppression d'une application (ordre inverse) :**
1. Stripe — retirer l'item de la subscription
2. SpiceDB — supprimer les relations
3. PostgreSQL — supprimer l'enregistrement
4. K8s — Helm uninstall

**Principe :** commencer par ce qui peut casser, finir par ce qui ne casse pas. Chaque étape a un mécanisme de rollback identifié.

### Frontend Architecture

**Routing :** React Router (SPA classique)

**State management :** Redux Toolkit, slices par domaine fonctionnel :
- `auth` — état OIDC
- `organizations` — organisations utilisateur
- `projects` — projets par organisation
- `catalog` — catalogue d'applications disponibles
- `deployments` — applications déployées et leur statut
- `billing` — subscription et coûts

**Composants :** Chakra UI v3 (migration depuis v2 de l'ancien projet — API différente, patterns conservés)

**Pas de dossier `utils/`** — anti-pattern identifié. La logique métier est dans les slices ou les composants.

### Infrastructure & Deployment

**Monorepo :**
```
france-nuage/
├── console/          # Frontend React/Chakra/Redux
├── controlplane/     # Rust workspace (inclut schema.zed SpiceDB)
├── node-sdk/         # Codegen TS + services partagés
├── protocol/         # .proto definitions
├── system-tests/     # E2E Playwright
└── .gitlab-ci.yml
```

**Dev local :** Docker-compose (PostgreSQL, Keycloak, SpiceDB, Traefik)

**Logging :** tracing (tokio ecosystem) avec tracing-subscriber, logs structurés JSON en prod

**CI/CD GitLab multi-stage :**
1. Security (audit dépendances)
2. Build (controlplane + console + node-sdk + codegen)
3. Lint (clippy, eslint, buf lint)
4. Test (cargo test — 100% coverage control plane)
5. Deploy staging
6. E2E (Playwright contre stack complète)

**Stratégie de test :**
- Control plane Rust : tests unitaires (100% coverage, traits pour mocking des intégrations externes)
- Frontend : aucun test unitaire (présentation pure, pas de logique métier isolable)
- E2E : Playwright contre la stack complète via docker-compose, teste les parcours utilisateur

### Decision Impact Analysis

**Séquence d'implémentation :**
1. Initialisation monorepo (cargo init workspace + npm create vite)
2. Protocol (.proto) + codegen setup (build.rs + protoc)
3. Auth middleware (Keycloak JWT validation)
4. Data model (tables + migrations sqlx)
5. Modules domaine (organizations, projects, catalog, deployments, billing)
6. Orchestration provisionnement (K8s-first)
7. Frontend console (Redux slices + pages)
8. E2E tests

**Dépendances inter-composants :**
- Le node-sdk dépend de protocol/ (codegen)
- La console dépend du node-sdk (types + services gRPC)
- Le controlplane dépend de protocol/ (tonic_build)
- Les system-tests dépendent du node-sdk + docker-compose

## Implementation Patterns & Consistency Rules

### Naming Patterns

**Base de données (PostgreSQL) :**
- Tables : `snake_case`, pluriel — `users`, `organizations`, `organization_user`
- Colonnes : `snake_case` — `created_at`, `organization_id`
- FK : `{table_singulier}_id` — `organization_id` référençant `organizations.id`
- Index : convention sqlx par défaut

**Rust (control plane) :**
- Modules/fichiers : `snake_case`
- Fonctions/variables : `snake_case`
- Structs/Enums/Traits : `PascalCase`
- Constantes : `SCREAMING_SNAKE_CASE`
- Crates : `snake_case` avec préfixe si besoin — `infra-stripe`, `infra-kube`

**TypeScript/React (console) :**
- Composants : `PascalCase` dans le code, fichiers `kebab-case.tsx` — `catalog-card.tsx` exporte `CatalogCard`
- Préfixe domaine sur les composants — `billing-form.tsx`, `deploy-form.tsx`, `catalog-card.tsx`
- Pages : suffixe `.page.tsx` — `catalog.page.tsx`
- Redux slices : suffixe `.slice.ts` — `catalog.slice.ts`
- Redux actions : `domaine/action` — `catalog/fetchApps`, `deployments/deploy`
- Hooks : `use-` préfixe, fichiers `kebab-case` — `use-app-dispatch.ts`
- Types/interfaces : `PascalCase` — `Organization`, `DeploymentStatus`

**Protobuf (.proto) :**
- Messages : `PascalCase` — `Organization`, `CreateProjectRequest`
- Champs : `snake_case` — `organization_id`, `created_at`
- Services : `PascalCase` — `ResourceManager`, `CatalogService`
- RPCs : `PascalCase` — `CreateOrganization`, `DeployApplication`

### Structure Patterns

**Backend — Architecture hexagonale par crates :**
```
controlplane/
├── Cargo.toml              # workspace
├── crates/
│   ├── server/             # Point d'entrée, gRPC handlers, middleware auth
│   │                       # Dépend de : domain, infra-*
│   ├── domain/             # Entités métier, ports (traits), use cases
│   │                       # Dépend de : RIEN (zéro dépendance externe)
│   ├── infra-db/           # Implémentation sqlx (queries, migrations)
│   │                       # Dépend de : domain, sqlx
│   ├── infra-stripe/       # Client Stripe HTTP
│   │                       # Dépend de : domain, reqwest
│   ├── infra-kube/         # Client K8s/Helm (kube-rs)
│   │                       # Dépend de : domain, kube
│   ├── infra-spicedb/      # Client SpiceDB gRPC
│   │                       # Dépend de : domain, tonic
│   └── core/               # Config, erreurs partagées, types communs
```

**Règles de dépendance (enforced par Cargo.toml) :**
- `domain` → zéro dépendance infra (le compilateur l'enforce)
- `infra-*` → dépendent de `domain` (implémentent les ports/traits)
- `server` → dépend de tout (assemble, injecte les implémentations)

**Frontend — Structure console :**
```
console/src/
├── main.tsx
├── router.tsx
├── store.ts
├── config.ts
├── components/           # Tous les composants, préfixés par domaine
│   ├── app-layout.tsx
│   ├── app-header.tsx
│   ├── app-sidebar.tsx
│   ├── billing-form.tsx
│   ├── catalog-card.tsx
│   ├── deploy-form.tsx
│   ├── project-switcher.tsx
│   ├── chakra/           # Customisations Chakra UI v3
│   └── ...
├── pages/                # Pages (routes), suffixe .page.tsx
├── features/             # Redux slices, suffixe .slice.ts
├── hooks/                # Hooks custom (use-app-dispatch, etc.)
├── services/             # Transport gRPC-web, clients RPC
├── middlewares/           # Redux middleware RPC
├── providers/            # Providers React (OIDC, user)
└── types/                # Types partagés
```

**Conventions :**
- Chaque dossier a un `index.ts` pour les exports
- Pas de dossier `utils/` — anti-pattern
- Pas de sous-dossiers par feature dans `components/` — préfixe domaine suffit

### Error Handling Patterns

**Backend — Erreurs Rust :**
- Enum `Error` central avec `#[derive(thiserror::Error)]`
- Conversion automatique via `impl From<Error> for tonic::Status`
- Chaque crate infra a son propre type d'erreur qui se convertit vers le type central
- Erreurs internes loguées via `tracing::error!`, message client générique (`"internal error"`)
- Aucun secret ni donnée sensible dans les messages d'erreur

**Mapping gRPC status codes :**

| Situation | gRPC Status Code |
|---|---|
| Token manquant/invalide | `UNAUTHENTICATED` |
| Pas de permission SpiceDB | `PERMISSION_DENIED` |
| Ressource non trouvée | `NOT_FOUND` |
| Validation formulaire échouée | `INVALID_ARGUMENT` |
| Doublon (slug, nom) | `ALREADY_EXISTS` |
| Helm install échoue | `FAILED_PRECONDITION` |
| Erreur Stripe / DB / interne | `INTERNAL` |

**Frontend — Intercepteur gRPC-web :**
- Intercepteur sur le `GrpcWebFetchTransport` qui applique le Bearer token + intercepte les erreurs
- Toast Chakra avec debounce 500ms (évite le spam si plusieurs appels échouent)
- `UNAUTHENTICATED` → logout automatique
- Mapping d'erreurs contextuelles (message backend → message user-friendly)
- Erreurs non gérées loguées + re-thrown

### Testing Patterns

**Control plane Rust — Tests unitaires + HTTP mocks :**
- 100% code coverage
- Mocking HTTP avec **mockito** (pas de trait mocking)
- Injection de client HTTP custom pointant vers le mock server
- Réponses pré-enregistrées réalistes (Stripe API, K8s API)
- Versions d'API ciblées (pas de `latest`) pour garantir la stabilité des mocks
- Les vraies implémentations tournent dans les tests (sérialisation, parsing, gestion d'erreurs)

**Frontend — Aucun test unitaire :**
- Le frontend est de la présentation pure, pas de logique métier isolable
- Pas de dossier `utils/`

**E2E — Playwright :**
- Stack complète via docker-compose (control plane + PG + Keycloak + SpiceDB)
- Tests des parcours utilisateur de bout en bout
- Projet séparé dans `system-tests/`

### Communication Patterns

**Redux actions :**
- Naming : `domaine/verbe` — `catalog/fetchApps`, `deployments/deploy`, `organizations/create`
- Async : Redux Toolkit `createAsyncThunk` avec appels gRPC via les services du node-sdk
- État de chargement : `status: 'idle' | 'loading' | 'succeeded' | 'failed'` dans chaque slice

**gRPC services (node-sdk) :**
- Transport configuré une fois avec intercepteur auth + error handling
- Services typés générés par protobuf-ts
- Le node-sdk expose des fonctions service qui encapsulent les appels gRPC

### Enforcement Guidelines

**Tous les agents IA DOIVENT :**
- Respecter les frontières de crates : `domain` n'importe JAMAIS de dépendance infra
- Préfixer les composants React par leur domaine fonctionnel
- Implémenter `From<Error> for tonic::Status` pour tout nouveau type d'erreur
- Utiliser mockito avec injection de client HTTP pour les tests (pas de trait mocking)
- Ne jamais créer de dossier `utils/` ni de fichiers utilitaires génériques
- Ne jamais exposer de secrets dans les messages d'erreur ou les logs

**Vérification :**
- `cargo check` enforce les frontières de crates (erreur de compilation si violation)
- `cargo test` avec couverture — tout code non testé est un bug
- `clippy` en CI pour les conventions Rust
- `eslint` en CI pour les conventions TypeScript

## Project Structure & Boundaries

### Complete Project Directory Structure

```
france-nuage/
├── .gitlab-ci.yml                    # Pipeline CI/CD multi-stage
├── .env.example                      # Variables d'environnement (template)
├── docker-compose.yml                # Dev local (PG, Keycloak, SpiceDB, Traefik)
├── docker-compose.ci.yml             # CI (stack complète pour E2E)
├── keycloak-realm.json               # Configuration realm Keycloak
│
├── protocol/                         # Source de vérité : .proto definitions
│   ├── buf.yaml                      # Lint rules pour les .proto
│   ├── resourcemanager.proto         # Organizations + Projects
│   ├── catalog.proto                 # Catalogue d'applications
│   ├── deployment.proto              # Provisionnement + gestion apps déployées
│   └── billing.proto                 # Facturation Stripe
│
├── controlplane/                     # Backend Rust (workspace hexagonal)
│   ├── Cargo.toml                    # Workspace members
│   ├── crates/
│   │   ├── server/                   # Point d'entrée + gRPC handlers
│   │   │   ├── Cargo.toml
│   │   │   ├── build.rs             # tonic_build (compile les .proto)
│   │   │   └── src/
│   │   │       ├── main.rs          # Démarrage serveur
│   │   │       ├── lib.rs
│   │   │       ├── config.rs        # Configuration (env vars)
│   │   │       ├── router.rs        # Router gRPC + middleware stack
│   │   │       ├── middleware/      # Auth JWT, logging, tonic-web
│   │   │       │   ├── mod.rs
│   │   │       │   └── auth.rs      # Validation JWT Keycloak
│   │   │       └── handlers/        # Implémentations gRPC services
│   │   │           ├── mod.rs
│   │   │           ├── resource_manager.rs  # Orgs + Projects
│   │   │           ├── catalog.rs           # Catalogue
│   │   │           ├── deployment.rs        # Provisionnement
│   │   │           └── billing.rs           # Facturation
│   │   │
│   │   ├── domain/                   # Entités métier + ports (ZÉRO dépendance infra)
│   │   │   ├── Cargo.toml           # Dépend de : core uniquement
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── entities/        # Structs métier
│   │   │       │   ├── mod.rs
│   │   │       │   ├── user.rs
│   │   │       │   ├── organization.rs
│   │   │       │   ├── project.rs
│   │   │       │   ├── catalog_app.rs
│   │   │       │   ├── deployment.rs
│   │   │       │   └── subscription.rs
│   │   │       └── ports/           # Traits (contrats pour les adapters)
│   │   │           ├── mod.rs
│   │   │           ├── persistence.rs    # Port DB
│   │   │           ├── billing.rs        # Port Stripe
│   │   │           ├── orchestrator.rs   # Port K8s/Helm
│   │   │           └── authorization.rs  # Port SpiceDB
│   │   │
│   │   ├── infra-db/                # Adapter PostgreSQL (sqlx)
│   │   │   ├── Cargo.toml           # Dépend de : domain, sqlx
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── users.rs
│   │   │       ├── organizations.rs
│   │   │       ├── projects.rs
│   │   │       ├── catalog.rs
│   │   │       ├── deployments.rs
│   │   │       └── migrations/      # sqlx migrate
│   │   │
│   │   ├── infra-stripe/            # Adapter Stripe HTTP
│   │   │   ├── Cargo.toml           # Dépend de : domain, reqwest
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── client.rs        # Client HTTP Stripe
│   │   │       ├── customers.rs
│   │   │       ├── subscriptions.rs
│   │   │       └── tests/           # Tests avec mockito
│   │   │           ├── mod.rs
│   │   │           ├── fixtures/    # Réponses JSON Stripe pré-enregistrées
│   │   │           ├── customers_test.rs
│   │   │           └── subscriptions_test.rs
│   │   │
│   │   ├── infra-kube/              # Adapter K8s/Helm (kube-rs)
│   │   │   ├── Cargo.toml           # Dépend de : domain, kube
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── client.rs        # Client K8s
│   │   │       ├── helm.rs          # Helm install/uninstall
│   │   │       └── tests/
│   │   │           ├── mod.rs
│   │   │           ├── fixtures/    # Réponses JSON K8s pré-enregistrées
│   │   │           └── helm_test.rs
│   │   │
│   │   ├── infra-spicedb/           # Adapter SpiceDB gRPC
│   │   │   ├── Cargo.toml           # Dépend de : domain, tonic
│   │   │   ├── schema.zed           # Schéma ReBAC SpiceDB
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── client.rs        # Client SpiceDB
│   │   │       ├── permissions.rs   # Check/write permissions
│   │   │       └── relationships.rs # Write relationships
│   │   │
│   │   └── core/                    # Types partagés, erreurs, config
│   │       ├── Cargo.toml           # Dépend de : RIEN
│   │       └── src/
│   │           ├── lib.rs
│   │           ├── error.rs         # Enum Error + From<Error> for tonic::Status
│   │           └── config.rs        # Types de configuration partagés
│
├── node-sdk/                         # SDK TypeScript partagé (console + system-tests)
│   ├── package.json
│   ├── tsconfig.json
│   └── src/
│       ├── index.ts                 # Re-exports
│       ├── generated/               # Code généré par protobuf-ts
│       │   └── rpc/
│       │       ├── resourcemanager.ts
│       │       ├── resourcemanager.client.ts
│       │       ├── catalog.ts
│       │       ├── catalog.client.ts
│       │       ├── deployment.ts
│       │       ├── deployment.client.ts
│       │       ├── billing.ts
│       │       └── billing.client.ts
│       ├── models/                  # Modèles domaine TypeScript
│       │   ├── index.ts
│       │   ├── organization.ts
│       │   ├── project.ts
│       │   ├── catalog-app.ts
│       │   ├── deployment.ts
│       │   └── subscription.ts
│       ├── fixtures/                # Générateurs de données test (Faker)
│       │   ├── index.ts
│       │   ├── organization.ts
│       │   ├── project.ts
│       │   └── deployment.ts
│       ├── services/                # Clients service
│       │   ├── index.ts
│       │   ├── transport.ts         # Transport gRPC-web
│       │   ├── resource-manager.ts
│       │   ├── catalog.ts
│       │   ├── deployment.ts
│       │   └── billing.ts
│       └── types/
│           └── index.ts
│
├── console/                          # Frontend React SPA
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── index.html
│   └── src/
│       ├── main.tsx
│       ├── router.tsx
│       ├── store.ts
│       ├── config.ts
│       ├── constants.ts
│       ├── components/
│       │   ├── index.ts
│       │   ├── app-layout.tsx
│       │   ├── app-header.tsx
│       │   ├── app-sidebar.tsx
│       │   ├── page-guard.tsx
│       │   ├── catalog-card.tsx
│       │   ├── deploy-form.tsx
│       │   ├── deployment-table.tsx
│       │   ├── project-switcher.tsx
│       │   ├── billing-summary.tsx
│       │   └── chakra/
│       │       ├── index.ts
│       │       ├── provider.tsx
│       │       └── toaster.tsx
│       ├── pages/
│       │   ├── index.ts
│       │   ├── login.page.tsx
│       │   ├── oidc-redirect.page.tsx
│       │   ├── home.page.tsx
│       │   ├── catalog.page.tsx
│       │   ├── deploy.page.tsx
│       │   ├── projects.page.tsx
│       │   └── billing.page.tsx
│       ├── features/
│       │   ├── index.ts
│       │   ├── authentication.slice.ts
│       │   ├── organizations.slice.ts
│       │   ├── projects.slice.ts
│       │   ├── catalog.slice.ts
│       │   ├── deployments.slice.ts
│       │   └── billing.slice.ts
│       ├── hooks/
│       │   ├── index.ts
│       │   ├── use-app-dispatch.ts
│       │   ├── use-app-selector.ts
│       │   └── use-controlplane-data.ts
│       ├── services/
│       │   ├── index.ts
│       │   └── transport.rpc.ts
│       ├── middlewares/
│       │   ├── index.ts
│       │   └── rpc.ts
│       ├── providers/
│       │   ├── index.ts
│       │   └── user-provider.tsx
│       └── types/
│           ├── index.ts
│           └── routes.ts
│
└── system-tests/                     # E2E Playwright
    ├── package.json
    ├── playwright.config.ts
    ├── tsconfig.json
    └── tests/
        ├── base.ts                  # Setup partagé
        ├── pages/                   # Page Object Model
        │   ├── base.page.ts
        │   ├── login.page.ts
        │   ├── oidc.page.ts
        │   ├── home.page.ts
        │   ├── catalog.page.ts
        │   └── deploy.page.ts
        └── cases/                   # Scénarios de test
            ├── security/
            │   └── authentication.spec.ts
            ├── catalog/
            │   └── browse.spec.ts
            ├── deployments/
            │   ├── deploy.spec.ts
            │   └── delete.spec.ts
            └── billing/
                └── subscription.spec.ts
```

### Architectural Boundaries

**Règles de dépendance des crates (enforced par Cargo.toml) :**
```
core         → (rien)
domain       → core
infra-db     → domain, sqlx, core
infra-stripe → domain, reqwest, core
infra-kube   → domain, kube, core
infra-spicedb → domain, tonic, core
server       → domain, infra-db, infra-stripe, infra-kube, infra-spicedb, core, tonic
```

**Frontières API (gRPC services) :**

| Proto Service | Crate handler | FRs couvertes |
|---|---|---|
| `ResourceManager` | `server/handlers/resource_manager.rs` | FR5-FR12 (orgs + projets) |
| `CatalogService` | `server/handlers/catalog.rs` | FR13-FR17 (catalogue) |
| `DeploymentService` | `server/handlers/deployment.rs` | FR18-FR26 (provision + gestion) |
| `BillingService` | `server/handlers/billing.rs` | FR27-FR30 (facturation) |

**Frontières transverses (middleware, pas de proto) :**

| Concern | Localisation | FRs couvertes |
|---|---|---|
| Auth JWT Keycloak | `server/middleware/auth.rs` | FR1-FR4 |
| Authz SpiceDB | `infra-spicedb/` appelé par les handlers | FR31-FR33 |

### Requirements to Structure Mapping

| FR | Proto Service | Backend crate(s) | Frontend slice + page |
|---|---|---|---|
| FR1-4 (Auth) | — (middleware) | `server/middleware/auth.rs` | `authentication.slice.ts`, `login.page.tsx` |
| FR5-9 (Orgs) | `ResourceManager` | `infra-db/organizations.rs`, `domain/entities/organization.rs` | `organizations.slice.ts` |
| FR10-12 (Projets) | `ResourceManager` | `infra-db/projects.rs`, `domain/entities/project.rs` | `projects.slice.ts`, `projects.page.tsx` |
| FR13-17 (Catalogue) | `CatalogService` | `infra-db/catalog.rs`, `domain/entities/catalog_app.rs` | `catalog.slice.ts`, `catalog.page.tsx` |
| FR18-23 (Provisionnement) | `DeploymentService` | `infra-kube/helm.rs`, `infra-db/deployments.rs` | `deployments.slice.ts`, `deploy.page.tsx` |
| FR24-26 (Apps déployées) | `DeploymentService` | `infra-db/deployments.rs`, `infra-kube/helm.rs` | `deployments.slice.ts`, `home.page.tsx` |
| FR27-30 (Billing) | `BillingService` | `infra-stripe/subscriptions.rs` | `billing.slice.ts`, `billing.page.tsx` |
| FR31-33 (Authz) | — (transverse) | `infra-spicedb/permissions.rs` | — (transparent) |

### Data Flow

**Déploiement d'une application :**
```
Console → gRPC-web → tonic-web middleware → auth middleware (JWT)
→ DeploymentService handler
  → SpiceDB check permission (infra-spicedb)
  → K8s Helm install (infra-kube)
  → PostgreSQL insert deployment (infra-db, transaction)
  → SpiceDB write relationship (infra-spicedb)
  → Stripe add subscription item (infra-stripe)
← gRPC response (status + URL)
```

### Integration Points

**Intégrations externes :**

| Système | Crate adapter | Protocol | Mocking |
|---|---|---|---|
| PostgreSQL | `infra-db` | sqlx (TCP) | Base de test dédiée |
| Stripe | `infra-stripe` | HTTP REST | mockito (client HTTP injecté) |
| Kubernetes | `infra-kube` | HTTP (kube-rs) | mockito (client HTTP injecté) |
| SpiceDB | `infra-spicedb` | gRPC | Instance SpiceDB de test |
| Keycloak | `server/middleware` | HTTP (JWKS) | mockito ou Keycloak de test |

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility :** Toutes les technologies sont dans l'écosystème tokio async (backend) et l'écosystème React/Vite (frontend). Aucun conflit de version identifié. tonic-web s'intègre nativement dans tonic. mockito est compatible avec reqwest pour le mocking HTTP.

**Pattern Consistency :** Les conventions de naming suivent les standards de chaque langage (snake_case Rust/SQL, camelCase TS, PascalCase composants). La codegen protobuf-ts traduit automatiquement entre les conventions proto et TypeScript. La chaîne d'erreurs (thiserror → tonic::Status → intercepteur frontend → toast) est cohérente de bout en bout.

**Structure Alignment :** L'architecture hexagonale par crates Rust enforce les frontières au compile-time. Le dossier `protocol/` partagé garantit un contrat API unique entre backend et frontend. La chaîne console → node-sdk → protocol → controlplane est clairement définie.

### Requirements Coverage ✅

**Functional Requirements :** 33/33 FRs ont un support architectural identifié. Chaque FR est mappée sur un service proto, un crate backend, et une slice/page frontend.

**Non-Functional Requirements :** Toutes les NFRs sont adressées architecturalement. La sécurité (JWT middleware, pas de secrets), la qualité (100% coverage, < 500 lignes), et la fiabilité (K8s-first, compensation) sont couvertes par des décisions concrètes.

### Implementation Readiness ✅

**Decision Completeness :** Toutes les décisions critiques sont documentées avec versions vérifiées. Les patterns d'implémentation couvrent le naming, la structure, les erreurs, les tests, et les communications.

**Structure Completeness :** L'arborescence projet est complète jusqu'au niveau fichier. Les frontières de crates sont explicites avec leurs règles de dépendance. Le mapping FR → fichiers est complet.

**Pattern Completeness :** Les conventions de naming, la gestion d'erreurs, la stratégie de test, et l'organisation Redux sont documentées avec des exemples concrets et des anti-patterns identifiés.

### Gap Analysis

**Aucun gap critique.** Les gaps identifiés (schema SpiceDB détaillé, messages proto, schema SQL, flux Stripe) seront affinés naturellement lors de l'implémentation, guidés par les FRs et les décisions architecturales documentées ici.

### Architecture Completeness Checklist

**✅ Requirements Analysis**
- [x] Contexte projet analysé en profondeur
- [x] Échelle et complexité évaluées (moyenne, 5 intégrations externes)
- [x] Contraintes techniques identifiées (stack ferme du project-context)
- [x] Préoccupations transverses mappées (auth, orchestration, codegen, observabilité, testabilité)

**✅ Architectural Decisions**
- [x] Décisions critiques documentées avec versions (9 décisions, toutes vérifiées)
- [x] Stack technologique entièrement spécifiée (Rust + React, toutes versions)
- [x] Patterns d'intégration définis (K8s-first, compensation, tonic-web)
- [x] Stratégie de test définie (mockito HTTP, Playwright E2E, pas de tests unitaires frontend)

**✅ Implementation Patterns**
- [x] Conventions de naming (DB, Rust, TS, Proto)
- [x] Patterns de structure (hexagonal crates, console flat components)
- [x] Patterns de communication (gRPC status codes, Redux actions, intercepteurs)
- [x] Patterns de process (error handling thiserror, testing mockito)

**✅ Project Structure**
- [x] Arborescence complète définie (monorepo 5 projets)
- [x] Frontières de composants établies (crates hexagonaux, Cargo.toml enforce)
- [x] Points d'intégration mappés (5 systèmes externes, protocol/mocking)
- [x] Mapping exigences → structure complet (33 FRs → fichiers)

### Architecture Readiness Assessment

**Overall Status:** READY FOR IMPLEMENTATION

**Confidence Level:** High — stack éprouvée (patterns de l'ancien projet), décisions fermes du project-context, architecture hexagonale enforced par le compilateur.

**Key Strengths:**
- Frontières de crates enforced au compile-time (anti-spaghetti)
- Tests HTTP réalistes (mockito) plutôt que des mocks abstraits
- Orchestration K8s-first qui simplifie la compensation
- Codegen bidirectionnelle proto → Rust + TypeScript
- Patterns éprouvés de l'ancien projet adaptés au nouveau scope

**Areas for Future Enhancement:**
- Schema SpiceDB détaillé (schema.zed) à écrire
- Messages proto à détailler (RPCs, request/response)
- Schema SQL à définir (migrations sqlx)
- UX design pour préciser les interactions console
- Monitoring et observabilité à approfondir post-MVP

### Implementation Handoff

**AI Agent Guidelines:**
- Suivre toutes les décisions architecturales exactement comme documentées
- Respecter les frontières de crates (`domain` = zéro dépendance infra)
- Utiliser les conventions de naming de chaque couche
- Tester avec mockito + injection client HTTP (pas de trait mocking)
- Ne jamais créer de dossier `utils/`
- Implémenter `From<Error> for tonic::Status` pour tout nouveau type d'erreur

**Première priorité d'implémentation :**
1. Initialisation monorepo (cargo init workspace + npm create vite)
2. Setup protocol/ + codegen (build.rs + protoc --ts_out)
3. Docker-compose dev local (PG, Keycloak, SpiceDB, Traefik)
