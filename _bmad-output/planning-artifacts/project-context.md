# France-nuage - Contexte Projet

**Date:** 2026-02-25
**Auteur:** Robin Straub (CEO & CTO)

## Vision

Permettre aux TPE/PME/ETI francaises de deployer leur SI complet (collaboratif, CRM, analytics...) sur un cloud souverain, simplement, sans competences techniques, avec la garantie de pouvoir tout reinternaliser.

### Strategie

Approche **top-down** : commencer par les couches hautes (apps/conteneurs sur K8s) pour generer du CA rapidement, puis redescendre progressivement vers les couches basses (infra, materiel) avec les revenus generes. L'objectif long terme reste un cloud 100% francais, 100% reinternalisable depuis les couches materielles.

### Timing

Contexte geopolitique (tensions US/EU, Cloud Act), prise de conscience croissante des dirigeants sur la dependance aux GAFAM. Dirigeants qui cherchent des alternatives a Google Workspace et Microsoft 365.

### Cible

- TPE, PME, ETI francaises
- Profils non-techniques : dirigeants, responsables IT sans expertise cloud
- DevOps (pour la partie administration)

### Proposition de valeur

Un point d'entree unique et simple pour un SI souverain complet. Pas besoin d'aller chercher Nextcloud ici, Odoo la, Metabase ailleurs - tout est centralise, pre-package, et accessible a un dirigeant non-technique. Heberge en France, sans compromis sur la souverainete.

---

## Pivot par rapport a l'ancien projet

| Avant (ancien scope) | Apres (V2) |
|---|---|
| Cloud complet : materiel -> VM -> containers | Uniquement couche applicative/conteneur sur K8s |
| Control plane gerant VMs, physique, CAS, Proxmox | Control plane "passe-plat" vers K8s + billing Stripe |
| Cible power users | Cible utilisateurs non-techniques |
| Deploiement libre/avance | Catalogue d'apps pre-packagees (Helm charts) |
| Pas de console web aboutie | Console web avec formulaires dynamiques |

---

## Architecture technique - Decisions fermes

### Stack backend (Control Plane)
- **Langage :** Rust (edition 2024)
- **API :** gRPC (tonic/prost) avec codegen TypeScript via @protobuf-ts
- **ORM :** Fabrique (ORM custom de Robin, SQL-first, type-safe, PostgreSQL)
- **Base de donnees :** PostgreSQL via sqlx
- **Objectif :** 100% code coverage en Rust

### Stack frontend (Console Web)
- **Framework :** React (pas de meta-framework type Next.js)
- **UI :** Chakra UI
- **State management :** Redux Toolkit
- **Build :** Vite
- **Transport gRPC :** @protobuf-ts/grpcweb-transport (binary format)
- **Codegen :** protoc avec @protobuf-ts/plugin -> types TypeScript generes depuis les .proto

### Authentification
- **Keycloak** (serveur deja en production)
- OIDC via oidc-client-ts cote frontend
- JWT validation cote backend
- Synchronizer Keycloak -> DB locale + SpiceDB

### Autorisation
- **SpiceDB** (Zanzibar-like, ReBAC - Relation Based Access Control)
- Schema hierarchique : organization -> project -> ressources
- Permissions en cascade via relations parent

### Billing
- **Stripe** - forfait mensuel simple
- 1 application = 1 produit Stripe = 1 prix/mois
- Mois commence est du
- Pas d'usage-based billing

### Infrastructure
- **Kubernetes** gere par l'equipe DevOps (pas par le control plane)
- **Helm charts** prepares par les DevOps, declares dans le control plane
- Le control plane recupere le Helm chart et l'applique sur K8s avec les valeurs utilisateur
- Docker-compose pour le dev local (PG, Keycloak, SpiceDB, Traefik)

---

## Fonctionnement du catalogue d'applications

1. L'equipe DevOps maintient un repository de Helm charts (packaging des applications)
2. Chaque Helm chart est declare dans le control plane (en base de donnees) avec les variables a remplacer
3. Le frontend genere dynamiquement un formulaire a partir des variables declarees
4. L'utilisateur remplit le formulaire (nom, email, options de config)
5. Le control plane applique le Helm chart sur K8s avec les valeurs saisies
6. L'application est provisionnee et accessible

---

## Ancien projet plateforme - Elements a reutiliser

**Repo :** /Users/robinstraub/Projects/france-nuage/plateforme (aussi sur GitLab: getbunker-france-nuage/france-nuage/plateforme)

### Patterns et architecture a conserver
- **gRPC-first API** : .proto -> codegen Rust (tonic/prost) + TypeScript (@protobuf-ts)
- **3 modes de service** dans le SDK : rpc (prod), api (mock HTTP), mock (in-memory)
- **Middleware gRPC** avec injection automatique du Bearer token
- **Synchronizer** Keycloak -> DB + SpiceDB
- **Schema ReBAC hierarchique** (org -> project -> ressources)
- **gRPC Reflection** pour le discovery
- **buf.yaml** pour le linting des .proto
- **Structure frontend** : React + Chakra UI + Redux Toolkit + Vite
- **oidc-client-ts** pour l'auth Keycloak
- **@tanstack/react-table** pour les tableaux de donnees
- **CI/CD GitLab** multi-stage (security, build, lint, test, deploy, e2e)

### Structure de l'ancien projet
```
plateforme/
├── console/          # React/Chakra/Redux frontend
├── controlplane/     # Rust gRPC backend (workspace multi-crates)
├── node-sdk/         # SDK TypeScript partage (codegen + services)
├── protocol/         # Proto definitions
├── authz/            # SpiceDB schema (schema.zed)
├── helm/             # Helm charts Kubernetes
├── infrastructure/   # IaC et CI/CD
├── system-tests/     # E2E Playwright
├── bruno/            # Tests API
├── adr/              # Architecture Decision Records
└── .gitlab-ci.yml    # Pipeline CI/CD
```

### Schema SpiceDB de l'ancien projet (authz/schema.zed)
```zed
definition organization {
  relation member: service_account | user
  relation parent: organization
  permission get = member + parent->member
  permission invite_member = member + parent->member
}
definition project {
  relation parent: folder | organization
  permission get = parent->get
  permission create_instance = get
}
```

### Proto services de l'ancien projet
- **compute.proto** : Hypervisors, Instances, Zones (a remplacer par le catalogue Helm)
- **resourcemanager.proto** : Organizations, Projects (a conserver/adapter)
- **iam.proto** : Invitations (a conserver/adapter)
- **infrastructure.proto** : Datacenters, ZeroTrustNetworks (a supprimer)

### Base de donnees de l'ancien projet (schema.hcl)
Tables pertinentes a conserver/adapter : users, organizations, projects, invitations, service_accounts, organization_user, organization_service_account, operations
Tables a supprimer : instances, hypervisors, zones, zero_trust_networks, zero_trust_network_types, infrastructure

### Codegen setup de l'ancien projet
- **Rust** : tonic_prost_build dans build.rs, genere aussi les file descriptors pour gRPC Reflection
- **TypeScript** : `protoc --ts_out` avec @protobuf-ts/plugin, optimize_code_size
- **Node SDK** : package partage entre console et system-tests

### Ce qu'on laisse tomber
- Tout ce qui est VM/hyperviseur/Proxmox
- Infrastructure physique (datacenters, zones, zero-trust networks)
- Operation worker (lie aux VMs)
- Les slices Redux liees aux VMs (hypervisors, instances, infrastructure)

---

## Nouveau controlplane (brouillon recent)

**Repo :** /Users/robinstraub/Projects/robinstraub/controlplane

Brouillon recent de Robin, pas une reference mais contient des idees utiles :
- Integration Stripe complete (customers, products, prices, subscriptions)
- Module CaaS basique (listing namespaces K8s via kube-rs)
- Utilisation de Fabrique ORM
- Architecture trait-based pour l'abstraction des services
- Pattern App<Billing, Caas> avec injection de dependances

---

## Site vitrine france-nuage.fr

- Positionnement : "Le post-cloud" - cloud souverain francais
- 3 datacenters en France (Vendee, Nantes)
- Offre SIOU existante (Directus, Odoo, Metabase, n8n, Nextcloud, Vaultwarden, etc.)
- Pricing : Starter (gratuit), Pro, Enterprise
- Documentation structuree (Diataxis)
- Contact : bonjour@france-nuage.fr

---

## Organisation du projet

- **Open source** : le code sera publie en open source
- **Specs versionnees** : les documents BMAD (PRD, architecture, etc.) sont commites dans le repo
- **Issues** : utilisation d'issues pour la transparence
- **Repo prive** dans un premier temps, publication ouverte ensuite
- **Multi-poste** : Robin travaille sur deux ordinateurs (perso + boulot), le repo est le moyen de synchronisation

---

## Business Metrics

- **CA actuel :** 1 500 EUR/mois (clients existants qui utilisent des apps hebergees, sans console)
- **Objectif fin 2026 :** 10 000 EUR/mois
- Les clients actuels n'ont jamais utilise la console web - conversion a faire

## Curation du catalogue

La qualite de France-nuage depend autant de la curation des briques open source que du control plane.
Criteres de selection : experience utilisateur comparable aux GAFAM.

**Decisions prises :**
- OnlyOffice > Collabora (interface superieure)
- Nextcloud ecarte (interface insuffisante)

**Catalogue cible MVP :** 10 applications couvrant les besoins SI essentiels

## Naming

- Ne pas parler de "V2" - c'est France-nuage tout court
