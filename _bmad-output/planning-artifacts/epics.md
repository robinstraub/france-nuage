---
stepsCompleted: [1, 2, 3, 4]
lastStep: 4
status: 'complete'
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/architecture.md
  - _bmad-output/planning-artifacts/ux-design-specification.md
---

# france-nuage - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for france-nuage, decomposing the requirements from the PRD, UX Design, and Architecture into implementable stories.

## Protocole de développement

Ce protocole s'applique à **toutes les stories** du projet.

### Approche TDD — Tests d'abord, implémentation ensuite

1. **RED** — Écrire les tests E2E Playwright qui expriment les acceptance criteria de la story. Les tests échouent (la fonctionnalité n'existe pas encore).
2. **GREEN** — Implémenter le minimum pour que les tests passent.
3. **REFACTOR** — Nettoyer le code sans casser les tests.

### Règles

- Aucune story user-facing n'est considérée terminée sans tests E2E Playwright qui passent
- Les tests backend Rust (unitaires, mockito) sont écrits en parallèle de l'implémentation
- Les tests existants ne doivent JAMAIS être supprimés ou modifiés pour faire passer du nouveau code — si un test existant échoue, c'est une régression à corriger
- Chaque test `describe` correspond à une story, chaque `test` correspond à un acceptance criteria

### Lisibilité pour contrôle humain

- Les noms de tests sont en français, descriptifs : `test("un utilisateur peut créer une organisation")`
- Les blocs `describe` portent le nom de la story : `describe("Story 1.5 : Création et consultation des organisations")`
- Reporter HTML Playwright activé pour revue humaine visuelle
- Screenshot automatique sur échec
- La CI produit un rapport accessible (artifact GitLab)

### Non-régression

- Avant chaque merge, TOUS les tests E2E existants doivent passer
- Un test qui passe est un contrat : il garantit un comportement utilisateur permanent
- L'agent IA n'a pas le droit de modifier un test existant sans justification explicite dans le commit

## Requirements Inventory

### Functional Requirements

- FR1 : Un utilisateur peut s'inscrire et se connecter via Keycloak (OIDC), y compris via fédération d'identité Google
- FR2 : Un utilisateur authentifié peut se déconnecter de la console
- FR3 : Le control plane peut valider un token d'authentification Keycloak sur chaque requête API
- FR4 : Le control plane peut identifier l'utilisateur et ses organisations à partir de son token
- FR5 : Un utilisateur peut créer une organisation
- FR6 : Un utilisateur peut consulter les organisations dont il est membre
- FR7 : La hiérarchie parent-enfant des organisations est gérée en interne par l'équipe France-nuage (non exposée aux utilisateurs clients)
- FR8 : Un admin de l'organisation racine peut consulter les organisations enfants, leurs projets et leurs applications déployées
- FR9 : L'organisation racine "France-nuage" est parente de toutes les organisations clientes
- FR10 : Un utilisateur peut créer un projet au sein de son organisation
- FR11 : Un utilisateur peut consulter les projets de son organisation
- FR12 : Un utilisateur déploie des applications dans le contexte d'un projet
- FR13 : Un utilisateur peut consulter le catalogue des applications disponibles
- FR14 : Un utilisateur peut voir le nom, la description et l'équivalent GAFAM de chaque application du catalogue
- FR15 : Le catalogue est alimenté par des métadonnées Helm charts stockées dans le système
- FR16 : Un utilisateur voit pour chaque application les variables configurables (nom, type, description, valeur par défaut, contraintes de validation)
- FR17 : Un opérateur DevOps peut déclarer un nouveau Helm chart et ses variables directement dans le système
- FR18 : Un utilisateur peut déployer une application depuis le catalogue dans un de ses projets
- FR19 : Un utilisateur voit un formulaire dynamique généré à partir des variables configurables du Helm chart sélectionné
- FR20 : Le formulaire valide les saisies utilisateur selon les contraintes définies avant soumission
- FR21 : Le control plane peut appliquer un Helm chart sur Kubernetes avec les valeurs saisies par l'utilisateur
- FR22 : L'utilisateur reçoit un feedback de statut sur le provisionnement (en cours, succès, échec)
- FR23 : L'utilisateur peut consulter l'URL d'accès de son application une fois déployée
- FR24 : Un utilisateur peut consulter la liste de ses applications déployées avec leur statut
- FR25 : Un utilisateur peut supprimer une application déployée (avec confirmation)
- FR26 : La suppression d'une application déclenche le retrait du Helm release sur K8s
- FR27 : Le premier déploiement d'une application déclenche la création d'un customer Stripe et la saisie des informations de paiement
- FR28 : Le déploiement d'une application crée une subscription Stripe (si première app) ou ajoute un item à la subscription existante
- FR29 : La suppression d'une application retire l'item correspondant de la subscription Stripe
- FR30 : Un utilisateur peut consulter la liste de ses applications et leur coût mensuel associé
- FR31 : Le control plane vérifie les permissions SpiceDB avant toute opération sur une ressource
- FR32 : Un admin d'une organisation parente hérite automatiquement des permissions sur les organisations enfants
- FR33 : Au MVP, tout membre d'une organisation a accès complet aux ressources de cette organisation

### NonFunctional Requirements

- NFR1 : Toutes les communications API sont authentifiées (authentification Keycloak obligatoire)
- NFR2 : Le control plane ne stocke que des identifiants de référence Stripe, pas de secrets ni de données de paiement
- NFR3 : Aucun secret n'est exposé dans les réponses API ou les logs
- NFR4 : Les données utilisateur restent hébergées en France (RGPD)
- NFR5 : 100% code coverage sur le control plane Rust
- NFR6 : Aucun module ne dépasse 500 lignes de code
- NFR7 : Les .proto sont la source de vérité pour les contrats API : toute modification régénère les types Rust et TypeScript
- NFR8 : Chaque fonction publique est documentée ; les conventions de nommage et patterns sont documentés
- NFR9 : Le provisionnement d'une application qui échoue ne laisse pas de ressources orphelines (Helm release, subscription Stripe)
- NFR10 : Les opérations Stripe et K8s sont idempotentes ou compensables en cas d'échec partiel

### Additional Requirements

- Architecture hexagonale par crates Rust : domain (zéro dépendance infra), infra-db, infra-stripe, infra-kube, infra-spicedb, server, core
- Orchestration K8s-first : Helm install d'abord (seul point faillible), puis PG, SpiceDB, Stripe en dernier — compensation en cascade inversée
- Codegen proto bidirectionnelle : .proto → Rust (tonic_build) + TypeScript (@protobuf-ts/plugin)
- Middleware auth JWT comme intercepteur gRPC tonic
- tonic-web middleware intégré (pas de proxy gRPC-web externe)
- Docker-compose pour dev local (PG, Keycloak, SpiceDB, Traefik, kind)
- Monorepo : console, controlplane, node-sdk, protocol, system-tests
- Starter backend : cargo init workspace from scratch
- Starter frontend : Vite react-ts template
- Console desktop-first, sidebar 280px fixe → Drawer sous breakpoint `md` (768px)
- Formulaires 100% dynamiques générés depuis les variables Helm chart
- WCAG 2.1 AA via Chakra UI v3 natif
- E2E Playwright + axe-core pour tests accessibilité
- Customer Stripe = flux séparé avant premier déploiement (hors tunnel provisionnement)
- CI/CD GitLab multi-stage : security, build, lint, test, deploy, E2E
- HTML sémantique : nav, main, header, fieldset/legend pour formulaires
- Pas de dossier utils/ (anti-pattern)
- Pas de tests unitaires frontend — E2E Playwright uniquement
- Tests backend : mockito HTTP mocking avec injection client HTTP custom
- Approche TDD : tests E2E écrits AVANT l'implémentation, tests existants = contrats permanents

### FR Coverage Map

| FR | Epic | Description |
|-----|------|-------------|
| FR1 | Epic 1 | Inscription/connexion Keycloak OIDC |
| FR2 | Epic 1 | Déconnexion |
| FR3 | Epic 1 | Validation token API |
| FR4 | Epic 1 | Identification user/orgs depuis token |
| FR5 | Epic 1 | Création organisation |
| FR6 | Epic 1 | Consultation organisations |
| FR7 | Epic 4 | Hiérarchie parent-enfant interne |
| FR8 | Epic 4 | Admin racine consulte orgs enfants |
| FR9 | Epic 4 | Org racine parente de toutes |
| FR10 | Epic 1 | Création projet |
| FR11 | Epic 1 | Consultation projets |
| FR12 | Epic 2 | Déploiement dans contexte projet |
| FR13 | Epic 2 | Consultation catalogue |
| FR14 | Epic 2 | Métadonnées app (nom, description, GAFAM) |
| FR15 | Epic 2 | Catalogue alimenté par Helm charts |
| FR16 | Epic 2 | Variables configurables visibles |
| FR17 | Epic 2 | DevOps déclare Helm chart en BDD |
| FR18 | Epic 2 | Déploiement depuis catalogue |
| FR19 | Epic 2 | Formulaire dynamique Helm |
| FR20 | Epic 2 | Validation formulaire |
| FR21 | Epic 2 | Helm apply sur K8s |
| FR22 | Epic 2 | Feedback statut provisionnement |
| FR23 | Epic 2 | URL d'accès post-déploiement |
| FR24 | Epic 3 | Liste apps déployées avec statut |
| FR25 | Epic 3 | Suppression app (confirmation) |
| FR26 | Epic 3 | Retrait Helm release K8s |
| FR27 | Epic 2 | Création customer Stripe |
| FR28 | Epic 2 | Subscription Stripe (création/ajout item) |
| FR29 | Epic 3 | Retrait item Stripe à la suppression |
| FR30 | Epic 3 | Consultation coûts mensuels |
| FR31 | Epic 4 | Vérification permissions SpiceDB |
| FR32 | Epic 4 | Héritage permissions via parent |
| FR33 | Epic 4 | Accès complet par org au MVP |

## Epic List

### Epic 1 : Authentification & Espace de travail
L'utilisateur peut se connecter via Keycloak, accéder à la console, créer son organisation et ses projets. Cet epic inclut l'initialisation du monorepo, l'infrastructure de tests E2E, la codegen, le docker-compose, et le shell de la console (sidebar, header, layout).
**FRs couvertes :** FR1, FR2, FR3, FR4, FR5, FR6, FR10, FR11

### Epic 2 : Catalogue & Déploiement d'applications
L'utilisateur peut parcourir le catalogue d'applications, sélectionner une app, remplir le formulaire dynamique, payer via Stripe, et déclencher le déploiement sur Kubernetes. Couvre le parcours complet depuis la découverte jusqu'à l'app fonctionnelle.
**FRs couvertes :** FR12, FR13, FR14, FR15, FR16, FR17, FR18, FR19, FR20, FR21, FR22, FR23, FR27, FR28

### Epic 3 : Gestion des applications & Facturation
L'utilisateur peut voir la liste de ses applications déployées avec leur statut, supprimer celles qu'il n'utilise plus (avec annulation automatique de l'abonnement Stripe), et consulter sa facturation mensuelle.
**FRs couvertes :** FR24, FR25, FR26, FR29, FR30

### Epic 4 : Autorisation & Administration
Le control plane vérifie les permissions SpiceDB avant chaque opération. La hiérarchie parent-enfant des organisations permet à l'admin France-nuage de consulter les organisations clientes, leurs projets et leurs applications. Au MVP, tout membre d'une org a accès complet.
**FRs couvertes :** FR7, FR8, FR9, FR31, FR32, FR33

---

## Epic 1 : Authentification & Espace de travail

L'utilisateur peut se connecter via Keycloak, accéder à la console, créer son organisation et ses projets. Après cet epic, un utilisateur a un espace de travail prêt pour déployer des applications.

### Story 1.1 : Initialisation du monorepo et environnement de développement

As a developer,
I want the monorepo structure and development environment configured,
So that I can start building features immediately.

**Acceptance Criteria:**

**Given** the project repository is cloned
**When** I run `docker-compose up`
**Then** PostgreSQL, Keycloak, SpiceDB, and Traefik start successfully
**And** Keycloak is accessible with a pre-configured realm

**Given** the cargo workspace is initialized
**When** I run `cargo build`
**Then** the project compiles with crates: server, domain, infra-db, infra-stripe, infra-kube, infra-spicedb, core

**Given** the protocol/ directory contains a .proto file
**When** I run `cargo build` (backend) or the protoc codegen script (frontend)
**Then** Rust types are generated via tonic_build
**And** TypeScript types are generated via @protobuf-ts/plugin

**Given** the console/ directory is initialized with Vite react-ts
**When** I run `npm run dev`
**Then** the React development server starts with HMR
**And** Chakra UI v3, Redux Toolkit, and oidc-client-ts are installed

**Given** the node-sdk/ and system-tests/ directories are initialized
**When** I inspect the project structure
**Then** the monorepo matches the architecture document structure

### Story 1.2 : Infrastructure de tests E2E et cluster Kubernetes de test

As a developer,
I want the E2E test infrastructure ready with a local K8s cluster,
So that I can write failing tests before implementing features (TDD).

**Acceptance Criteria:**

**Given** le docker-compose est lancé
**When** je démarre la stack de test
**Then** un cluster Kubernetes local est disponible via kind (Kubernetes in Docker)
**And** le control plane peut communiquer avec ce cluster via kube-rs
**And** le cluster est capable de recevoir des Helm install

**Given** le cluster K8s de test
**When** une suite de tests se termine
**Then** les namespaces et Helm releases créés pendant les tests sont nettoyés automatiquement
**And** le cluster est dans un état propre pour la prochaine suite

**Given** le projet system-tests/ est configuré
**When** je lance `npx playwright test`
**Then** Playwright démarre avec la configuration de base (viewport desktop 1280x720, viewport tablette 768x1024)
**And** axe-core est intégré pour la détection automatique des violations WCAG AA
**And** le reporter HTML est activé pour la revue humaine

**Given** les tests E2E
**When** j'écris un nouveau test
**Then** je dispose d'un helper d'authentification Keycloak (login programmatique, pas via UI)
**And** je dispose d'un Page Object Model de base (BasePage, LoginPage)
**And** je dispose de fixtures pour créer/nettoyer des organisations et projets de test

**Given** la CI/CD GitLab
**When** le pipeline E2E s'exécute
**Then** docker-compose lance la stack complète (PG, Keycloak, SpiceDB, Traefik, control plane, kind)
**And** les tests Playwright s'exécutent contre cette stack
**And** le rapport HTML est publié comme artifact GitLab

### Story 1.3 : Shell de la console et authentification frontend

FR1, FR2

As a user,
I want to sign in to the France-nuage console via Keycloak and see the application shell,
So that I can access my cloud workspace securely.

**Acceptance Criteria:**

**Given** I am not authenticated
**When** I navigate to the console URL
**Then** I am redirected to the Keycloak login page

**Given** I am on the Keycloak login page
**When** I sign in with valid credentials (or register a new account)
**Then** I am redirected back to the console
**And** the OIDC token is stored in memory via oidc-client-ts

**Given** I am authenticated
**When** the console loads
**Then** I see the application shell with a sidebar (nav), header (logo, avatar), and main content area
**And** the sidebar contains navigation items
**And** my name or email appears in the header

**Given** I am authenticated
**When** I click the logout button
**Then** I am logged out of both the console and Keycloak
**And** I am redirected to the login page

**Given** my OIDC token has expired
**When** I perform any action in the console
**Then** the token is silently refreshed via oidc-client-ts
**And** if refresh fails, I am redirected to the login page

### Story 1.4 : Middleware d'authentification et création utilisateur (backend)

FR3, FR4

As a system,
I want to validate JWT tokens and identify users on every API request,
So that only authenticated users can access the control plane.

**Acceptance Criteria:**

**Given** a gRPC request arrives at the control plane
**When** the request has no Authorization header
**Then** the request is rejected with gRPC status UNAUTHENTICATED

**Given** a gRPC request arrives with an invalid or expired JWT token
**When** the middleware validates the token against Keycloak JWKS
**Then** the request is rejected with gRPC status UNAUTHENTICATED

**Given** a gRPC request arrives with a valid JWT token
**When** the middleware validates the token
**Then** the user's Keycloak ID and email are extracted from the token
**And** the request proceeds to the handler with user context injected

**Given** a valid JWT token is presented for the first time (user not in database)
**When** the middleware processes the request
**Then** a new user record is created in the users table with the Keycloak ID and email from the token ID
**And** subsequent requests with the same token do not create duplicate users

**Given** the users table
**When** I inspect the migration
**Then** it contains columns: id (UUID), keycloak_id (unique), email, name, created_at

### Story 1.5 : Création et consultation des organisations

FR5, FR6

As a user,
I want to create an organization and see the organizations I belong to,
So that I can set up my workspace on France-nuage.

**Acceptance Criteria:**

**Given** I am authenticated
**When** I create a new organization with a name (e.g. "Cabinet Sophie Martin")
**Then** the organization is created in the database
**And** I am added as a member of the organization (organization_user table)
**And** the organization appears in my organizations list

**Given** the organization is created
**When** I view my organizations
**Then** I see all organizations where I am a member
**And** each organization shows its name

**Given** I try to create an organization with an empty name
**When** I submit the form
**Then** the request is rejected with gRPC status INVALID_ARGUMENT

**Given** I have no organizations
**When** I access the console after login
**Then** I am prompted to create my first organization (onboarding flow)

**Given** the organizations table
**When** I inspect the migration
**Then** it contains columns: id (UUID), name, parent_id (nullable FK to organizations), created_at

**Given** the organization_user table
**When** I inspect the migration
**Then** it contains columns: organization_id (FK), user_id (FK), created_at

### Story 1.6 : Création et consultation des projets

FR10, FR11

As a user,
I want to create projects within my organization,
So that I can organize my deployed applications.

**Acceptance Criteria:**

**Given** I am a member of an organization
**When** I create a new project with a name (e.g. "Outils collaboratifs")
**Then** the project is created in the database under my organization
**And** the project appears in my projects list

**Given** I belong to an organization with projects
**When** I view my projects
**Then** I see all projects for the currently selected organization
**And** each project shows its name

**Given** I have no projects in my organization
**When** I access the console
**Then** I am prompted to create my first project (onboarding flow)

**Given** I have at least one project
**When** I use the project switcher in the header
**Then** I can switch between my projects
**And** the selected project becomes the active context for deployments

**Given** the projects table
**When** I inspect the migration
**Then** it contains columns: id (UUID), name, organization_id (FK), created_at

---

## Epic 2 : Catalogue & Déploiement d'applications

L'utilisateur peut découvrir les applications disponibles, configurer et déployer une application sur Kubernetes, et payer son abonnement. Après cet epic, un utilisateur a une application fonctionnelle hébergée en France.

### Story 2.1 : Modèle de données du catalogue

FR15, FR17

As a DevOps operator,
I want to declare Helm charts and their configurable variables in the system,
So that applications appear in the user catalog.

**Acceptance Criteria:**

**Given** the database is initialized
**When** a DevOps operator inserts a catalog app record (SQL or script)
**Then** the record contains: id, name, description, gafam_equivalent, helm_chart_ref, price_monthly, icon_url, created_at

**Given** a catalog app exists
**When** a DevOps operator inserts variable records for that app
**Then** each variable contains: id, catalog_app_id (FK), name, type (string/number/boolean/email), label, description, default_value, validation_rules (JSON), display_order

**Given** the CatalogService proto is defined
**When** I inspect the .proto file
**Then** it contains RPCs: ListCatalogApps, GetCatalogApp
**And** the response messages include app metadata and variables

**Given** a catalog app with variables is declared in the database
**When** the CatalogService.ListCatalogApps RPC is called
**Then** all apps with their variables are returned

### Story 2.2 : Consultation du catalogue

FR13, FR14, FR16

As a user,
I want to browse the application catalog and see details about each app,
So that I can choose which applications to deploy.

**Acceptance Criteria:**

**Given** I am authenticated and have a project selected
**When** I navigate to the catalog page
**Then** I see a grid of application cards
**And** each card shows the app name, description, GAFAM equivalent, and monthly price

**Given** I am on the catalog page
**When** I click on an application card
**Then** I am navigated to the deployment page for that application
**And** I can see the app's configurable variables before deploying

**Given** the catalog is empty
**When** I navigate to the catalog page
**Then** I see a message indicating no applications are available

### Story 2.3 : Formulaire dynamique de déploiement

FR12, FR19, FR20

As a user,
I want to fill out a dynamically generated form based on the app's Helm chart variables,
So that I can configure my application before deployment.

**Acceptance Criteria:**

**Given** I selected an app from the catalog
**When** I view the deployment page
**Then** I see a form dynamically generated from the app's configurable variables
**And** each field shows its label, description, and default value
**And** the fields are ordered by display_order

**Given** a variable has type "email"
**When** I enter an invalid email address
**Then** the form shows a validation error on that field
**And** the deploy button is disabled

**Given** a variable has type "string" with a validation rule (e.g. min_length: 3)
**When** I enter a value that violates the constraint
**Then** the form shows a validation error
**And** the deploy button is disabled

**Given** all required fields are filled with valid values
**When** I review the form
**Then** I see a price recap showing the monthly cost of this app
**And** the deploy button is enabled

**Given** an app has no configurable variables
**When** I view the deployment page
**Then** I see only the price recap and the deploy button (no form fields)

### Story 2.4 : Enregistrement du moyen de paiement Stripe

FR27

As a user,
I want to register my payment method before my first deployment,
So that my subscription can be billed automatically.

**Acceptance Criteria:**

**Given** I am deploying for the first time (no Stripe customer exists for my organization)
**When** I click the deploy button
**Then** I am redirected to a Stripe payment flow (Checkout or Elements) to enter my card
**And** a Stripe customer is created for my organization
**And** the customer_id is stored in the database

**Given** I already have a Stripe customer with a valid payment method
**When** I deploy a new application
**Then** I am NOT asked for payment information again
**And** the deployment proceeds directly

**Given** I am on the Stripe payment flow
**When** I cancel or close the payment form
**Then** I am returned to the deployment page
**And** no customer is created
**And** the deployment does not proceed

### Story 2.5 : Provisionnement complet d'une application

FR18, FR21, FR22, FR23, FR28

As a user,
I want to deploy an application on Kubernetes and have my subscription updated automatically,
So that the app is accessible and I am billed correctly.

**Acceptance Criteria:**

**Given** I filled the deployment form with valid values and have a valid payment method
**When** I click the deploy button
**Then** the provisioning process starts in this order:
1. Helm chart is installed on Kubernetes with user-provided values
2. A deployment record is created in PostgreSQL
3. A Stripe subscription item is created (or added to existing subscription)
**And** I see a status indicator showing "Déploiement en cours"

**Given** the Helm install succeeds
**When** all subsequent steps complete
**Then** I see the status change to "Actif"
**And** I see the access URL of my deployed application

**Given** the Helm install fails
**When** the error is caught
**Then** no database record is created, no Stripe item is added
**And** I see the status "Échec" with a message "Le déploiement a échoué. Vous pouvez réessayer ou contacter notre équipe."

**Given** the Helm install succeeds but the PostgreSQL insert fails
**When** the error is caught
**Then** the Helm release is uninstalled (rollback)
**And** I see the status "Échec"

**Given** all steps succeed
**When** I inspect the Stripe subscription
**Then** it contains an item for this app with the correct price

**Given** the deployments table
**When** I inspect the migration
**Then** it contains columns: id (UUID), project_id (FK), catalog_app_id (FK), status (enum: provisioning/active/failed/deleting), helm_release_name, access_url, values_json, created_at

---

## Epic 3 : Gestion des applications & Facturation

L'utilisateur peut gérer le cycle de vie complet de ses applications : voir leur statut, les supprimer, et comprendre ce qu'il paie. Après cet epic, l'utilisateur a une autonomie complète sur son SI.

### Story 3.1 : Liste des applications déployées

FR24

As a user,
I want to see all my deployed applications with their status,
So that I can monitor my infrastructure at a glance.

**Acceptance Criteria:**

**Given** I have deployed applications in my project
**When** I navigate to the home/dashboard page
**Then** I see a table listing all my deployed apps
**And** each row shows: app name, status (Actif/En cours/Échec), access URL (clickable), deployment date

**Given** I have apps in multiple projects
**When** I switch projects via the project switcher
**Then** the table updates to show only apps for the selected project

**Given** I have no deployed applications
**When** I view the dashboard
**Then** I see a message inviting me to deploy my first application from the catalog

### Story 3.2 : Suppression d'une application

FR25, FR26, FR29

As a user,
I want to delete a deployed application,
So that I stop paying for services I no longer need.

**Acceptance Criteria:**

**Given** I have a deployed application with status "Actif"
**When** I click the delete button on that application
**Then** a confirmation dialog appears asking "Êtes-vous sûr de vouloir supprimer [app name] ?"
**And** the dialog shows the billing impact ("Votre abonnement sera réduit de X EUR/mois")

**Given** I confirm the deletion
**When** the deletion process starts
**Then** the operations happen in this order:
1. Stripe subscription item is removed
2. PostgreSQL deployment record is deleted
3. Helm release is uninstalled from K8s
**And** the app status changes to "Suppression en cours" then disappears from the list

**Given** I cancel the deletion dialog
**When** the dialog closes
**Then** nothing happens and the app remains unchanged

**Given** the Helm uninstall fails during deletion
**When** the error is caught
**Then** the app status shows "Échec suppression"
**And** I can retry the deletion

### Story 3.3 : Consultation de la facturation

FR30

As a user,
I want to see what I pay per month for each application,
So that I can manage my budget.

**Acceptance Criteria:**

**Given** I have deployed applications
**When** I navigate to the billing page
**Then** I see the total monthly cost
**And** I see a list of each deployed application with its individual monthly cost

**Given** I have no deployed applications
**When** I navigate to the billing page
**Then** I see a total of 0 EUR and an empty list

---

## Epic 4 : Autorisation & Administration

Le control plane vérifie les permissions SpiceDB avant chaque opération. La hiérarchie parent-enfant des organisations permet à l'admin France-nuage de consulter les organisations clientes. Après cet epic, la plateforme est sécurisée et administrable.

### Story 4.1 : Schéma SpiceDB et vérification des permissions

FR31, FR33

As a system,
I want to verify permissions via SpiceDB before every resource operation,
So that users can only access resources they are authorized to manage.

**Acceptance Criteria:**

**Given** the SpiceDB schema is loaded (schema.zed)
**When** I inspect the schema
**Then** it defines: organization (with member relation), project (with parent→organization relation), deployment (with parent→project relation)
**And** permissions cascade via parent relations

**Given** a user is a member of an organization
**When** they perform any operation on a resource within that organization
**Then** SpiceDB checks the permission and allows the operation

**Given** a user is NOT a member of an organization
**When** they attempt to access a resource within that organization
**Then** the request is rejected with gRPC status PERMISSION_DENIED

**Given** a user creates an organization
**When** the organization is created
**Then** a SpiceDB relationship organization:{org_id}#member@user:{user_id} is written

**Given** a user creates a project
**When** the project is created
**Then** a SpiceDB relationship project:{project_id}#parent@organization:{org_id} is written

**Given** all existing endpoints (organizations, projects, catalog, deployments, billing)
**When** I audit the handlers
**Then** every handler that reads or writes a resource checks SpiceDB permissions before proceeding

### Story 4.2 : Hiérarchie des organisations

FR7, FR9

As a France-nuage operator,
I want to set up the organization hierarchy with a root organization,
So that the admin can oversee all client organizations.

**Acceptance Criteria:**

**Given** the system is initialized
**When** I inspect the database
**Then** a root organization "France-nuage" exists with parent_id = NULL

**Given** a user creates a new organization
**When** the organization is created
**Then** its parent_id is set to the root organization "France-nuage" ID
**And** a SpiceDB relationship organization:{new_org_id}#parent@organization:{root_org_id} is written

**Given** the organization hierarchy
**When** I query SpiceDB for permissions
**Then** members of a parent organization inherit permissions on child organizations

### Story 4.3 : Administration via l'organisation racine

FR8, FR32

As a France-nuage admin (member of the root organization),
I want to view all client organizations, their projects, and deployed applications,
So that I can provide support and monitor the platform.

**Acceptance Criteria:**

**Given** I am a member of the root organization "France-nuage"
**When** I navigate to the console
**Then** I can see all child organizations in my organizations list (via SpiceDB permission inheritance)

**Given** I select a client's organization
**When** I view their projects
**Then** I see all projects for that organization
**And** I can view their deployed applications with status

**Given** I am NOT a member of the root organization
**When** I try to access another organization
**Then** the request is rejected with gRPC status PERMISSION_DENIED
