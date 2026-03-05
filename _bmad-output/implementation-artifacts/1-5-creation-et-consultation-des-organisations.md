# Story 1.5: Création et consultation des organisations

Status: in-progress

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a user,
I want to create an organization and see the organizations I belong to,
so that I can set up my workspace on France-nuage.

## Acceptance Criteria

1. **Given** I am authenticated
   **When** I create a new organization with a name (e.g. "Cabinet Sophie Martin")
   **Then** the organization is created in the database
   **And** I am added as a member of the organization (organization_user table)
   **And** the organization appears in my organizations list

2. **Given** the organization is created
   **When** I view my organizations
   **Then** I see all organizations where I am a member
   **And** each organization shows its name

3. **Given** I try to create an organization with an empty name
   **When** I submit the form
   **Then** the request is rejected with gRPC status INVALID_ARGUMENT

4. **Given** I have no organizations
   **When** I access the console after login
   **Then** I am prompted to create my first organization (onboarding flow)

5. **Given** the organizations table
   **When** I inspect the migration
   **Then** it contains columns: id (UUID), name, parent_id (nullable FK to organizations), created_at

6. **Given** the organization_user table
   **When** I inspect the migration
   **Then** it contains columns: organization_id (FK), user_id (FK), created_at

## Tasks / Subtasks

- [ ] Task 1 : Migrations SQL — tables organizations et organization_user (AC: #5, #6)
  - [ ] Créer `controlplane/migrations/20260305_create_organizations.sql` avec table `organizations` (id UUID PK, name TEXT NOT NULL, parent_id UUID nullable FK self-ref, created_at TIMESTAMPTZ DEFAULT NOW())
  - [ ] Créer `controlplane/migrations/20260305_create_organization_user.sql` avec table `organization_user` (organization_id UUID FK, user_id UUID FK, created_at TIMESTAMPTZ DEFAULT NOW(), PRIMARY KEY (organization_id, user_id))
  - [ ] Index sur `organization_user.user_id` pour les lookups rapides

- [ ] Task 2 : Entité domain Organization (AC: #1, #2)
  - [ ] Créer `controlplane/crates/domain/src/entities/organization.rs` — struct Organization { id: Uuid, name: String, parent_id: Option<Uuid>, created_at: DateTime<Utc> }
  - [ ] Ajouter `pub mod organization` dans `domain/src/entities/mod.rs`

- [ ] Task 3 : Port OrganizationRepository (AC: #1, #2, #3)
  - [ ] Créer `controlplane/crates/domain/src/ports/organization_repository.rs` — trait OrganizationRepository: Send + Sync
  - [ ] Méthodes : `create(&self, organization: &Organization, user_id: Uuid) -> Result<Organization, Error>` (crée l'org + l'entrée organization_user en transaction)
  - [ ] Méthode : `list_by_user(&self, user_id: Uuid) -> Result<Vec<Organization>, Error>`
  - [ ] Ajouter `pub mod organization_repository` dans `domain/src/ports/mod.rs`

- [ ] Task 4 : Enrichir le .proto ResourceManager (AC: #1, #2, #3)
  - [ ] Ajouter `CreateOrganizationRequest { string name = 1; }` et `CreateOrganizationResponse { Organization organization = 1; }`
  - [ ] Ajouter le RPC `CreateOrganization(CreateOrganizationRequest) returns (CreateOrganizationResponse)`
  - [ ] Vérifier la codegen Rust (cargo build) et TypeScript (protoc)

- [ ] Task 5 : Handler gRPC ResourceManagerService (AC: #1, #2, #3)
  - [ ] Créer `controlplane/crates/server/src/handlers/mod.rs` et `resource_manager.rs`
  - [ ] Implémenter `ResourceManagerService` avec les RPCs CreateOrganization et ListOrganizations
  - [ ] CreateOrganization : valider le nom non vide (sinon INVALID_ARGUMENT), appeler `authenticate()`, appeler `organization_repo.create()`
  - [ ] ListOrganizations : appeler `authenticate()`, appeler `organization_repo.list_by_user(user.id)`
  - [ ] Injecter les dépendances (OpenID, UserRepository, OrganizationRepository) dans la struct du service

- [ ] Task 6 : Brancher le service gRPC dans le serveur (AC: #1, #2)
  - [ ] Modifier `controlplane/crates/server/src/main.rs` pour construire et enregistrer le `ResourceManagerService` dans le router tonic
  - [ ] Configurer tonic-web middleware si pas déjà fait
  - [ ] Ajouter gRPC Reflection pour le service

- [ ] Task 7 : Implémentation infra-db — PgUserRepository + PgOrganizationRepository (AC: #1, #2, #5, #6)
  - [ ] Initialiser le crate infra-db : ajouter dépendances sqlx, domain, core dans Cargo.toml
  - [ ] Créer `controlplane/crates/infra-db/src/users.rs` — impl UserRepository for PgUserRepository (find_by_keycloak_id, create, find_or_create) avec sqlx
  - [ ] Créer `controlplane/crates/infra-db/src/organizations.rs` — impl OrganizationRepository for PgOrganizationRepository
  - [ ] `create()` : transaction SQL → INSERT organizations + INSERT organization_user
  - [ ] `list_by_user()` : SELECT organizations JOIN organization_user WHERE user_id = $1
  - [ ] Utiliser sqlx avec les types UUID et DateTime<Utc>
  - [ ] Exécuter `cargo sqlx prepare` pour générer les fichiers `.sqlx/` (mode offline pour la CI)

- [ ] Task 8 : Tests unitaires Rust 100% coverage (AC: #1, #2, #3)
  - [ ] Tests du handler : mock OrganizationRepository, tester CreateOrganization (happy path + nom vide + non authentifié)
  - [ ] Tests du handler : mock OrganizationRepository, tester ListOrganizations (happy path + liste vide)
  - [ ] Tests infra-db : utiliser une base PG de test ou mockito si HTTP-based
  - [ ] Noms de tests en français

- [ ] Task 9 : Codegen TypeScript et service node-sdk (AC: #1, #2)
  - [ ] Régénérer les types TypeScript depuis le .proto mis à jour
  - [ ] Créer/mettre à jour `node-sdk/src/services/resource-manager.ts` — fonctions createOrganization() et listOrganizations()
  - [ ] Exporter depuis `node-sdk/src/index.ts`

- [ ] Task 10 : Redux slice organizations (AC: #1, #2, #4)
  - [ ] Créer `console/src/features/organizations.slice.ts` — état : { items: Organization[], selected: Organization | null, status: 'idle' | 'loading' | 'succeeded' | 'failed' }
  - [ ] Actions async : `organizations/fetchAll` (ListOrganizations), `organizations/create` (CreateOrganization), `organizations/select`
  - [ ] Enregistrer le slice dans `console/src/store.ts`

- [ ] Task 11 : Page onboarding — création première organisation (AC: #4)
  - [ ] Créer `console/src/pages/onboarding.page.tsx` — formulaire simple : champ "Nom de votre organisation" + bouton "Créer"
  - [ ] Logique : après login, si ListOrganizations retourne une liste vide → redirect vers `/onboarding`
  - [ ] Après création réussie → redirect vers Home (ou vers la création de projet dans Story 1.6)
  - [ ] Toast de confirmation "Organisation créée"
  - [ ] Intégrer la route `/onboarding` dans le routeur (protégée par PageGuard)

- [ ] Task 12 : Créer le composant organization-switcher et l'intégrer dans le header (AC: #2)
  - [ ] Créer `console/src/components/organization-switcher.tsx` — composant Chakra `Menu.Root` / `Menu.Trigger` / `Menu.Content` / `Menu.Item` (pattern identique à l'ancien projet plateforme/console ProjectGlobalSwitcher)
  - [ ] Style du trigger : border, chevrons icon (LuChevronsUpDown), text ellipsis, focus ring
  - [ ] Menu.Content avec la liste des organisations, Menu.Item par org, dispatch `organizations/select` au changement
  - [ ] Modifier `console/src/components/app-header.tsx` pour intégrer le OrganizationSwitcher dans le header
  - [ ] Au chargement de la console, dispatcher `organizations/fetchAll` et sélectionner la première org
  - [ ] Si pas d'org → redirect vers `/onboarding` (AC #4)

- [ ] Task 13 : Tests E2E Playwright (AC: #1, #2, #3, #4)
  - [ ] Créer `system-tests/tests/cases/organizations.spec.ts` — describe("Story 1.5 : Création et consultation des organisations")
  - [ ] Test : "un utilisateur peut créer une organisation" (AC #1)
  - [ ] Test : "un utilisateur voit ses organisations" (AC #2)
  - [ ] Test : "la création d'une organisation avec un nom vide échoue" (AC #3)
  - [ ] Test : "un utilisateur sans organisation voit la page d'onboarding" (AC #4)
  - [ ] Test : "la page d'onboarding est accessible (WCAG AA)" — axe-core
  - [ ] Utiliser la fixture `actingAs` de base.ts pour l'authentification
  - [ ] Créer les Page Objects nécessaires (OnboardingPage)

- [ ] Task 14 : Vérification d'intégrité
  - [ ] Tous les tests E2E existants passent (non-régression)
  - [ ] `npm run lint` (Biome) passe sans erreur
  - [ ] `npm run build` passe sans erreur
  - [ ] `cargo clippy` passe sans warning
  - [ ] `cargo test` avec coverage 100%
  - [ ] CI locale via `/ci-local`

## Dev Notes

### Architecture & Patterns

**Flux gRPC CreateOrganization :**
1. Console → gRPC-web → tonic-web middleware
2. → Auth middleware (JWT validation via OpenID) → extraction User
3. → ResourceManagerService.CreateOrganization handler
4. → Validation : nom non vide (sinon INVALID_ARGUMENT)
5. → OrganizationRepository.create(org, user.id) → transaction PG (INSERT org + INSERT organization_user)
6. ← gRPC response avec l'organisation créée

**Flux gRPC ListOrganizations :**
1. Console → gRPC-web → tonic-web middleware
2. → Auth middleware → extraction User
3. → ResourceManagerService.ListOrganizations handler
4. → OrganizationRepository.list_by_user(user.id) → SELECT avec JOIN
5. ← gRPC response avec la liste des organisations

**Pattern de handler gRPC (établi dans Story 1.4) :**
```rust
pub struct ResourceManagerServiceImpl {
    openid: OpenID,
    user_repo: Arc<dyn UserRepository>,
    organization_repo: Arc<dyn OrganizationRepository>,
}

#[tonic::async_trait]
impl ResourceManagerService for ResourceManagerServiceImpl {
    async fn create_organization(
        &self,
        request: Request<CreateOrganizationRequest>,
    ) -> Result<Response<CreateOrganizationResponse>, Status> {
        let user = authenticate(&request, &self.openid, self.user_repo.as_ref()).await?;
        let req = request.into_inner();
        if req.name.trim().is_empty() {
            return Err(Error::InvalidArgument("name must not be empty".into()).into());
        }
        let org = Organization {
            id: Uuid::new_v4(),
            name: req.name,
            parent_id: None,
            created_at: Utc::now(),
        };
        let created = self.organization_repo.create(&org, user.id).await?;
        Ok(Response::new(CreateOrganizationResponse {
            organization: Some(created.into()),
        }))
    }
}
```

**Pattern d'implémentation infra-db (nouveau pour cette story) :**
```rust
pub struct PgOrganizationRepository {
    pool: PgPool,
}

#[async_trait::async_trait]
impl OrganizationRepository for PgOrganizationRepository {
    async fn create(&self, organization: &Organization, user_id: Uuid) -> Result<Organization, Error> {
        let mut tx = self.pool.begin().await.map_err(|e| Error::Internal(e.to_string()))?;
        sqlx::query("INSERT INTO organizations (id, name, parent_id, created_at) VALUES ($1, $2, $3, $4)")
            .bind(organization.id)
            .bind(&organization.name)
            .bind(organization.parent_id)
            .bind(organization.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        sqlx::query("INSERT INTO organization_user (organization_id, user_id, created_at) VALUES ($1, $2, NOW())")
            .bind(organization.id)
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Error::Internal(e.to_string()))?;
        Ok(organization.clone())
    }
}
```

**Onboarding frontend (UX Design Spec) :**
- Si l'utilisateur n'a pas d'organisation → page `/onboarding` avec formulaire simple
- Formulaire : un seul champ "Nom de votre organisation" + bouton "Créer"
- Après création → redirect vers Home (Story 1.6 ajoutera l'étape projet)
- Toast Chakra UI de confirmation

**Organization Switcher (pattern ancien projet plateforme) :**
- Composant `OrganizationSwitcher` basé sur Chakra UI `Menu` (pas un `<select>` HTML natif)
- Pattern identique au `ProjectGlobalSwitcher` de `/home/rstraub/Projects/france-nuage/plateforme/console/src/components/project-global-switcher.tsx`
- `Menu.Root` avec `positioning={{ placement: 'bottom-start' }}`
- `Menu.Trigger` stylé comme bouton : border, chevrons icon (`LuChevronsUpDown`), text ellipsis, focus ring
- `Menu.Content` avec `Menu.Item` pour chaque organisation
- Redux : dispatch `organizations/select` au changement
- Fallback : "Aucune organisation" si liste vide
- Placé dans le header entre le logo et l'avatar utilisateur

**Redux slice pattern :**
```typescript
// console/src/features/organizations.slice.ts
import { createAsyncThunk, createSlice } from '@reduxjs/toolkit';

export const fetchOrganizations = createAsyncThunk(
  'organizations/fetchAll',
  async () => { /* appel gRPC ListOrganizations */ }
);

export const createOrganization = createAsyncThunk(
  'organizations/create',
  async (name: string) => { /* appel gRPC CreateOrganization */ }
);
```

### Fichiers à créer

**Backend :**
- `controlplane/migrations/20260305_create_organizations.sql`
- `controlplane/migrations/20260305_create_organization_user.sql`
- `controlplane/crates/domain/src/entities/organization.rs`
- `controlplane/crates/domain/src/ports/organization_repository.rs`
- `controlplane/crates/server/src/handlers/mod.rs`
- `controlplane/crates/server/src/handlers/resource_manager.rs`
- `controlplane/crates/infra-db/src/organizations.rs`

**Frontend :**
- `console/src/features/organizations.slice.ts`
- `console/src/pages/onboarding.page.tsx`
- `console/src/components/organization-switcher.tsx`

**SDK :**
- `node-sdk/src/services/resource-manager.ts` (nouveau ou MAJ)

**Tests :**
- `system-tests/tests/cases/organizations.spec.ts`
- `system-tests/tests/pages/onboarding.page.ts`

### Fichiers à modifier

**Backend :**
- `protocol/france_nuage/resourcemanager/v1/resourcemanager.proto` — ajouter RPCs
- `controlplane/crates/domain/src/entities/mod.rs` — ajouter organization
- `controlplane/crates/domain/src/ports/mod.rs` — ajouter organization_repository
- `controlplane/crates/server/src/lib.rs` — ajouter mod handlers
- `controlplane/crates/server/src/main.rs` — brancher le service gRPC
- `controlplane/crates/server/Cargo.toml` — dépendances si nécessaire
- `controlplane/crates/infra-db/Cargo.toml` — ajouter dépendance sqlx
- `controlplane/crates/infra-db/src/lib.rs` — ajouter mod organizations

**Frontend :**
- `console/src/store.ts` — enregistrer organizations slice
- `console/src/components/app-header.tsx` — afficher org sélectionnée
- `console/src/App.tsx` ou `console/src/router.tsx` — ajouter route /onboarding
- `console/src/components/page-guard.tsx` — redirect vers /onboarding si pas d'org

**SDK :**
- `node-sdk/src/index.ts` — exporter le service resource-manager

### Conventions à respecter

- **Pas de dossier utils/** — anti-pattern
- **Biome** pour le lint/format frontend (pas eslint/prettier)
- **Noms de tests en français** : `test("un utilisateur peut créer une organisation")`
- **Code en anglais** : noms de variables, fonctions, composants, structs
- **100% code coverage Rust** (cargo-llvm-cov --fail-under-lines 100)
- **Tests E2E Playwright** : noms français, fixture `actingAs`, axe-core pour WCAG
- **Architecture hexagonale** : domain ne dépend jamais d'infra
- **gRPC status codes** : INVALID_ARGUMENT pour validation, UNAUTHENTICATED pour auth
- **HTML sémantique** : `<form>`, `<fieldset>`, `<legend>` pour le formulaire d'onboarding
- **Composants Chakra directs** — pas de wrappers customs inutiles

### Project Structure Notes

- Le dossier `handlers/` dans server est nouveau — c'est la première story qui ajoute un handler gRPC
- `infra-db` passe de placeholder vide à première implémentation réelle
- La connexion PostgreSQL (PgPool) devra être initialisée dans `main.rs` et injectée
- Le pattern d'injection est via `Arc<dyn Trait>` (établi dans auth avec `&dyn UserRepository`)
- Le UserRepository n'a PAS d'implémentation infra-db dans le code actuel (le find_or_create est dans auth/mock.rs pour les tests) — cette story est la première à implémenter un vrai accès BDD

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 1.5] — Acceptance criteria, FR5/FR6
- [Source: _bmad-output/planning-artifacts/architecture.md#Data Architecture] — Table users, organization_user, pattern hexagonal
- [Source: _bmad-output/planning-artifacts/architecture.md#Implementation Patterns] — Naming, error handling, testing
- [Source: _bmad-output/planning-artifacts/architecture.md#Project Structure] — Arborescence complète, frontières crates
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#User Journey Flows] — Parcours 1 Sophie, onboarding org → projet
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Flow Optimization Principles] — Onboarding en entonnoir
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Component Strategy] — project-switcher, page-guard
- [Source: controlplane/crates/server/src/auth/mod.rs] — Pattern authenticate(), extract_authorization_token()
- [Source: controlplane/crates/domain/src/ports/user_repository.rs] — Pattern trait repository
- [Source: controlplane/crates/core/src/error.rs] — Error enum, From<Error> for tonic::Status
- [Source: protocol/france_nuage/resourcemanager/v1/resourcemanager.proto] — Proto existant à enrichir
- [Source: /home/rstraub/Projects/france-nuage/plateforme/console/src/components/project-global-switcher.tsx] — Pattern Chakra Menu switcher (ancien projet)

### Previous Story Intelligence (Story 1.4)

**Patterns établis :**
- Auth middleware : `authenticate()` retourne un `User` depuis JWT + find_or_create
- Injection de dépendances : `&dyn UserRepository` passé aux fonctions
- Mock pattern : struct MockUserRepository implémente le trait pour les tests
- Tests avec mockito pour les endpoints HTTP (OIDC discovery, JWKS)
- 25 tests, 100% coverage, noms en français
- Error enum dans core avec conversion automatique vers tonic::Status

**Points d'attention :**
- Le UserRepository actuel n'a PAS d'implémentation sqlx réelle — seul MockUserRepository existe dans auth/mock.rs
- Il faudra aussi implémenter PgUserRepository dans infra-db pour que le serveur puisse fonctionner en réel
- Le main.rs actuel ne fait que tracing setup, pas encore de serveur gRPC → cette story doit le brancher
- Les crates infra-db, infra-stripe, infra-kube, infra-spicedb sont des placeholders vides

**Fichiers créés/modifiés dans Story 1.4 :**
- `controlplane/crates/core/src/claims.rs` — Claims JWT
- `controlplane/crates/domain/src/entities/user.rs` — User entity
- `controlplane/crates/domain/src/ports/user_repository.rs` — UserRepository trait
- `controlplane/crates/server/src/auth/` — mod.rs, openid.rs, mock.rs
- `controlplane/migrations/20260302_create_users.sql` — Table users

### Git Intelligence

**Commits récents :**
- `535eebd` feat(controlplane): middleware d'authentification et création utilisateur (Story 1.4) (#4)
- `cb3997b` feat(console): shell et authentification OIDC (Story 1.3) (#3)
- `04f8a93` feat(system-tests): infrastructure E2E Story 1.2 (#2)
- `44de7b6` feat: initialisation du monorepo et environnement de développement (#1)

**Convention de commit :** `feat(scope): description` — scope = composant principal (controlplane, console, system-tests)
**Convention de branche :** `us{N}` pour les feature branches → PR vers master

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
