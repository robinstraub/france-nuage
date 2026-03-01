# Story 1.2 : Infrastructure de tests E2E et cluster Kubernetes de test

Status: review

## Story

As a developer,
I want the E2E test infrastructure ready with a local K8s cluster,
So that I can write failing tests before implementing features (TDD).

## Acceptance Criteria

1. **Given** le docker-compose est lancé
   **When** je démarre la stack de test
   **Then** un cluster Kubernetes local est disponible via kind (Kubernetes in Docker)
   **And** le control plane peut communiquer avec ce cluster via kube-rs
   **And** le cluster est capable de recevoir des Helm install

2. **Given** le cluster K8s de test
   **When** une suite de tests se termine
   **Then** les namespaces et Helm releases créés pendant les tests sont nettoyés automatiquement
   **And** le cluster est dans un état propre pour la prochaine suite

3. **Given** le projet system-tests/ est configuré
   **When** je lance `npx playwright test`
   **Then** Playwright démarre avec la configuration de base (viewport desktop 1280x720, viewport tablette 768x1024)
   **And** axe-core est intégré pour la détection automatique des violations WCAG AA
   **And** le reporter HTML est activé pour la revue humaine

4. **Given** les tests E2E
   **When** j'écris un nouveau test
   **Then** je dispose d'un helper d'authentification Keycloak (login programmatique, pas via UI)
   **And** je dispose d'un Page Object Model de base (BasePage, LoginPage)
   **And** je dispose de fixtures pour créer/nettoyer des organisations et projets de test

5. **Given** la CI GitHub Actions
   **When** le pipeline E2E s'exécute
   **Then** docker-compose lance la stack complète (PG, Keycloak, SpiceDB, Traefik, control plane, console)
   **And** les tests Playwright s'exécutent contre cette stack
   **And** le rapport HTML est publié comme artifact GitHub Actions

## Tasks / Subtasks

- [x] Task 1 : Playwright — viewports et axe-core (AC: #3)
  - [ ] Ajouter `@axe-core/playwright` dans `system-tests/package.json` (devDependencies)
  - [ ] Modifier `system-tests/playwright.config.ts` : ajouter un projet `tablet` avec viewport 768x1024 en plus du `chromium` existant (desktop 1280x720)
  - [ ] Créer un helper `system-tests/tests/helpers/accessibility.ts` qui expose une fonction `verifierAccessibilite(page)` utilisant `AxeBuilder` avec les tags `['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa']`
  - [ ] Mettre à jour le Dockerfile system-tests si nécessaire (installer chromium deps)
  - [ ] Vérifier `npx playwright test` démarre avec les deux projets (chromium + tablet)

- [x] Task 2 : Auth helper Keycloak — login programmatique (AC: #4)
  - [ ] Ajouter un client `e2e-tests` dans `keycloak-realm.json` : public, `directAccessGrantsEnabled: true`, même redirectUris que `console`
  - [ ] Ajouter un utilisateur de test dans `keycloak-realm.json` (section `users`) : `testuser@france-nuage.test` / mot de passe `test123`
  - [ ] Créer `system-tests/tests/helpers/auth.ts` :
    - Fonction `obtenirTokenKeycloak(username, password)` : appel HTTP POST vers `${KEYCLOAK_URL}/realms/france-nuage/protocol/openid-connect/token` avec `grant_type=password`, `client_id=e2e-tests`
    - Fonction `injecterAuthentification(page, tokens)` : injecte les tokens dans le sessionStorage au format attendu par oidc-client-ts (`oidc.user:${authority}:${client_id}`)
    - Fonction `authentifierUtilisateur(page, { username, password })` : combine les deux (obtenir token + injecter)
  - [ ] Exposer le helper via `system-tests/tests/helpers/index.ts`
  - [ ] Ajouter les variables d'environnement nécessaires dans le docker-compose (déjà présentes : `KEYCLOAK_URL`, `KEYCLOAK_ADMIN`, `KEYCLOAK_ADMIN_PASSWORD`)

- [x] Task 3 : Page Object Model — BasePage et LoginPage (AC: #4)
  - [ ] Créer `system-tests/tests/pages/base.page.ts` : classe abstraite avec `page`, `goto(path)`, `verifierAccessibilite()` (qui appelle le helper axe-core)
  - [ ] Créer `system-tests/tests/pages/login.page.ts` : hérite de BasePage, locators pour le formulaire Keycloak (username, password, submit)
  - [ ] Refactorer `system-tests/tests/pages/home.page.ts` : extraire `HomePage` depuis `base.ts` vers son propre fichier, hériter de BasePage
  - [ ] Mettre à jour `system-tests/tests/base.ts` : importer les pages depuis `pages/`, enrichir la fixture `pages` avec `login` et `home`
  - [ ] Mettre à jour le test existant `home.spec.ts` pour vérifier qu'il passe toujours

- [x] Task 4 : Fixtures — création et nettoyage de données de test (AC: #4)
  - [ ] Créer `system-tests/tests/helpers/keycloak-admin.ts` :
    - Fonction `creerUtilisateurKeycloak(user)` : utilise l'API admin Keycloak pour créer un utilisateur
    - Fonction `supprimerUtilisateurKeycloak(userId)` : supprime l'utilisateur via l'API admin
    - Utilise les variables `KEYCLOAK_ADMIN` et `KEYCLOAK_ADMIN_PASSWORD` pour obtenir un admin token
  - [ ] Créer `system-tests/tests/helpers/cleanup.ts` :
    - Fonction `nettoyerApresTest()` : supprime les données créées pendant le test (via les API admin disponibles)
    - Pattern : chaque fixture retourne un `cleanup()` callback
  - [ ] Enrichir les fixtures `node-sdk` existantes si nécessaire (les fixtures `organization` et `user` existent déjà dans `node-sdk/src/fixtures/`)

- [x] Task 5 : k3s cluster — service docker-compose (AC: #1)
  - [ ] Créer `kind-config.yaml` à la racine du projet :
    - 1 control-plane node (suffisant pour les tests)
    - Port mapping pour accès depuis les containers docker-compose (API server)
  - [ ] Créer `scripts/kind-setup.sh` :
    - Vérifie que `kind` est installé
    - Crée le cluster `kind create cluster --name france-nuage-e2e --config kind-config.yaml`
    - Exporte le kubeconfig vers `./kubeconfig-e2e.yaml`
    - Modifie le kubeconfig pour utiliser le nom réseau Docker du control-plane node (au lieu de `127.0.0.1`)
  - [ ] Créer `scripts/kind-teardown.sh` :
    - Supprime le cluster `kind delete cluster --name france-nuage-e2e`
    - Supprime le fichier kubeconfig
  - [ ] Ajouter `kubeconfig-e2e.yaml` au `.gitignore`
  - [ ] Documenter dans le README ou les scripts la commande d'installation de kind : `go install sigs.k8s.io/kind@v0.31.0` ou via brew

- [x] Task 6 : k3s cluster — intégration docker-compose et k8s-cleanup.ts (AC: #1, #2)
  - [ ] Ajouter une variable d'environnement `KUBECONFIG` au service `controlplane` dans `docker-compose.yml` pointant vers le kubeconfig monté
  - [ ] Monter le fichier `kubeconfig-e2e.yaml` dans le container controlplane (volume bind mount)
  - [ ] Créer `system-tests/tests/helpers/k8s-cleanup.ts` :
    - Fonction `nettoyerNamespace(name)` : supprime un namespace K8s de test
    - Fonction `nettoyerHelmRelease(name, namespace)` : désinstalle un Helm release
    - Fonction `nettoyerRessourcesTest()` : supprime tous les namespaces commençant par `test-`
    - Ces fonctions seront utilisées en `afterAll` ou `afterEach` dans les tests futurs
  - [ ] Note : le nettoyage K8s sera utilisé à partir de la Story 2.5 (provisionnement), cette story prépare uniquement les helpers

- [x] Task 7 : CI — intégration k3s dans GitHub Actions (AC: #5)
  - [ ] Modifier `.github/workflows/ci.yml` : dans le job `system-tests`, ajouter une étape d'installation de kind avant le `docker compose up`
  - [ ] Ajouter l'étape de création du cluster kind (utiliser `helm/kind-action@v1` ou installer kind manuellement)
  - [ ] S'assurer que le kubeconfig est exporté et accessible par les containers docker-compose
  - [ ] Ajouter une étape de suppression du cluster kind dans le bloc `if: always()`
  - [ ] Vérifier que le rapport HTML Playwright est toujours uploadé comme artifact

- [x] Task 8 : Vérification d'intégrité (AC: tous)
  - [ ] Vérifier que `npx playwright test` passe avec les deux viewports (chromium + tablet)
  - [ ] Vérifier que le test existant `home.spec.ts` passe toujours
  - [ ] Vérifier que l'auth helper peut obtenir un token Keycloak (test de connexion)
  - [ ] Vérifier que le cluster kind est accessible (list namespaces)
  - [ ] Vérifier que la CI passe (lancer les checks CI en local via Docker avant push)

## Dev Notes

### Décisions architecturales critiques

- **Login programmatique Keycloak** (PAS via l'UI Keycloak dans Playwright) — le client `e2e-tests` avec `directAccessGrantsEnabled: true` permet d'appeler directement le token endpoint (Resource Owner Password Credentials grant). Les tokens obtenus sont injectés dans le sessionStorage du navigateur au format oidc-client-ts. Cela évite de naviguer dans l'UI Keycloak pour chaque test, rendant les tests plus rapides et moins fragiles.
- **Client `e2e-tests` séparé** du client `console` — le client `console` a `directAccessGrantsEnabled: false` et utilise PKCE, ce qui est correct pour la production. Le client `e2e-tests` active le Direct Access Grants uniquement pour les tests. Ne PAS modifier le client `console`.
- **axe-core intégré dans BasePage** — chaque Page Object hérite d'une méthode `verifierAccessibilite()` qui scanne la page pour les violations WCAG AA. Les tests peuvent l'appeler à des moments clés (après navigation, après interaction).
- **Kind cluster sur réseau Docker** — kind crée des containers Docker. Pour que le controlplane (dans docker-compose) puisse atteindre le kind cluster, le kubeconfig doit être modifié pour pointer vers l'IP Docker du node kind (pas `127.0.0.1`).
- **Pas de Makefile** — le projet n'a pas de Makefile (contrairement à ce que la story 1-1 indiquait). Utiliser des scripts shell dans `scripts/`.
- **Biome** (pas ESLint/Prettier) — le frontend utilise Biome. Les fichiers TypeScript dans system-tests doivent respecter les conventions Biome.
- **Noms de tests en français** — convention du projet. Exemples : `test("un utilisateur authentifié voit la page d'accueil")`, `describe("Story 1.3 : Shell de la console")`.

### Approche détaillée — Auth helper Keycloak

Le flux complet pour l'authentification programmatique :

```
1. system-tests appelle POST /realms/france-nuage/protocol/openid-connect/token
   Body: grant_type=password&client_id=e2e-tests&username=testuser@france-nuage.test&password=test123&scope=openid+profile+email
2. Keycloak retourne { access_token, id_token, refresh_token, ... }
3. Le helper construit l'objet User au format oidc-client-ts :
   {
     access_token: "...",
     id_token: "...",
     token_type: "Bearer",
     scope: "openid profile email",
     profile: { sub: "...", email: "testuser@france-nuage.test", ... },
     expires_at: <timestamp>
   }
4. Le helper injecte cet objet dans sessionStorage :
   clé = "oidc.user:http://keycloak:8080/realms/france-nuage:e2e-tests"
   valeur = JSON.stringify(user)
5. Le test navigue vers la console, qui trouve l'utilisateur en session
```

**Important :** l'URL de l'authority dans la clé sessionStorage doit correspondre EXACTEMENT à celle configurée dans la console (`VITE_OIDC_AUTHORITY`). En environnement docker-compose/CI, l'URL Keycloak vue par le navigateur (dans le container system-tests) est `http://keycloak:8080` mais la console est configurée avec `http://localhost:8080`. Il faut harmoniser cette configuration.

**Solution :** dans le docker-compose, pour les tests E2E, la console doit utiliser `VITE_OIDC_AUTHORITY=http://keycloak:8080/realms/france-nuage` (même URL que celle accessible depuis le container system-tests). Actuellement c'est `http://localhost:8080/realms/france-nuage` (pour le dev local navigateur). Il faudra peut-être un override docker-compose ou un `.env.test`.

### Approche détaillée — Kind cluster

**Installation de kind :**
- macOS : `brew install kind`
- Linux/CI : `go install sigs.k8s.io/kind@v0.31.0` ou download binaire
- CI GitHub Actions : utiliser `helm/kind-action@v1` qui installe kind + crée le cluster

**Configuration minimale (`kind-config.yaml`) :**
```yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
  - role: control-plane
    extraPortMappings:
      - containerPort: 30000
        hostPort: 30000
        protocol: TCP
```

**Réseau Docker :**
- kind crée son propre réseau Docker (`kind`)
- Le controlplane dans docker-compose est sur le réseau `france-nuage_default`
- Pour la communication, deux options :
  1. **Connecter le container kind au réseau docker-compose** : `docker network connect france-nuage_default <kind-node-container>`
  2. **Utiliser `--network` avec kind** : possible mais expérimental
- L'option 1 est la plus fiable, à faire dans le script `kind-setup.sh`

**Kubeconfig :**
- `kind get kubeconfig --name france-nuage-e2e > kubeconfig-e2e.yaml`
- Modifier le `server:` pour utiliser le nom du container kind au lieu de `127.0.0.1`
- Monter ce fichier dans le container controlplane

### Approche détaillée — axe-core

```typescript
import { AxeBuilder } from '@axe-core/playwright';

// Dans BasePage
async verifierAccessibilite() {
  const resultats = await new AxeBuilder({ page: this.page })
    .withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'])
    .analyze();
  return resultats;
}

// Dans un test
test("la page d'accueil est accessible", async ({ pages }) => {
  await pages.home.goto();
  const resultats = await pages.home.verifierAccessibilite();
  expect(resultats.violations).toEqual([]);
});
```

### Versions des dépendances (vérifiées février 2026)

| Outil | Version | Usage |
|---|---|---|
| @axe-core/playwright | 4.11.x | Tests d'accessibilité WCAG AA |
| @playwright/test | 1.52.0+ (garder la version actuelle) | Framework E2E |
| kind | v0.31.0 | Cluster K8s local pour les tests |
| helm/kind-action | v1 | GitHub Action pour kind en CI |

### Ce que cette story NE fait PAS

- Pas d'implémentation de handlers gRPC (Stories 1.4+)
- Pas de migrations SQL (Story 1.4+)
- Pas de pages frontend fonctionnelles (Story 1.3)
- Pas de déploiement Helm réel (Story 2.5) — seul le cluster kind est prêt
- Pas de tests E2E user-facing — seule l'infrastructure de test est en place
- Pas de tests d'authentification complets (Story 1.3 pour le frontend, 1.4 pour le backend)
- L'helper de nettoyage K8s est créé mais ne sera utilisé qu'à partir de la Story 2.5

### Fichiers créés/modifiés par cette story

**Fichiers CRÉÉS :**
```
kind-config.yaml                                    # Config cluster kind
scripts/kind-setup.sh                               # Création cluster + kubeconfig
scripts/kind-teardown.sh                            # Suppression cluster
system-tests/tests/helpers/auth.ts                  # Login programmatique Keycloak
system-tests/tests/helpers/accessibility.ts         # Helper axe-core WCAG AA
system-tests/tests/helpers/keycloak-admin.ts        # API admin Keycloak (CRUD users)
system-tests/tests/helpers/cleanup.ts               # Pattern cleanup après tests
system-tests/tests/helpers/k8s-cleanup.ts           # Nettoyage ressources K8s
system-tests/tests/helpers/index.ts                 # Barrel exports
system-tests/tests/pages/base.page.ts               # Page Object Model abstrait
system-tests/tests/pages/login.page.ts              # POM page login Keycloak
system-tests/tests/pages/home.page.ts               # POM page accueil (extrait de base.ts)
```

**Fichiers MODIFIÉS :**
```
system-tests/package.json                           # +@axe-core/playwright
system-tests/playwright.config.ts                   # +projet tablet 768x1024
system-tests/tests/base.ts                          # Refacto fixtures, imports pages/
system-tests/tests/cases/home.spec.ts               # Adapter aux nouveaux imports
system-tests/Dockerfile                             # Si deps supplémentaires nécessaires
keycloak-realm.json                                 # +client e2e-tests, +utilisateur test
docker-compose.yml                                  # +volume kubeconfig, +env KUBECONFIG
.github/workflows/ci.yml                            # +kind setup/teardown dans job E2E
.gitignore                                          # +kubeconfig-e2e.yaml
```

### Project Structure Notes

Structure de `system-tests/` après cette story :
```
system-tests/
├── Dockerfile
├── package.json
├── playwright.config.ts
├── tsconfig.json
└── tests/
    ├── base.ts                          # Fixtures Playwright custom
    ├── helpers/
    │   ├── index.ts                     # Barrel exports
    │   ├── auth.ts                      # Login programmatique Keycloak
    │   ├── accessibility.ts             # Helper axe-core WCAG AA
    │   ├── keycloak-admin.ts            # API admin Keycloak
    │   ├── cleanup.ts                   # Pattern cleanup
    │   └── k8s-cleanup.ts              # Nettoyage K8s (kind)
    ├── pages/
    │   ├── base.page.ts                # POM abstrait
    │   ├── home.page.ts                # POM page accueil
    │   └── login.page.ts              # POM page login
    └── cases/
        └── home.spec.ts                # Test existant (adapté)
```

### Conventions rappelées

| Contexte | Convention | Exemple |
|---|---|---|
| Noms de tests | Français, descriptif | `test("un utilisateur peut se connecter")` |
| Blocs describe | Nom de la story | `describe("Story 1.3 : Shell de la console")` |
| Fichiers POM | .page.ts suffix | `home.page.ts` |
| Fichiers helpers | kebab-case | `keycloak-admin.ts` |
| Pas de dossier utils/ | Anti-pattern | Utiliser `helpers/` pour les tests |
| Screenshots | Sur échec uniquement | Déjà configuré dans playwright.config.ts |
| Reporter | HTML | Pour revue humaine |

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 1.2]
- [Source: _bmad-output/planning-artifacts/architecture.md#Testing Patterns]
- [Source: _bmad-output/planning-artifacts/architecture.md#Infrastructure & Deployment]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Responsive Strategy — viewport 768px breakpoint]
- [Source: _bmad-output/planning-artifacts/prd.md#Non-Functional Requirements — WCAG 2.1 AA]
- [Source: _bmad-output/planning-artifacts/project-context.md#Ancien projet plateforme — structure system-tests]
- [Source: _bmad-output/implementation-artifacts/1-1-initialisation-du-monorepo-et-environnement-de-developpement.md#Completion Notes]

## Previous Story Intelligence (Story 1.1)

### Learnings à appliquer

- **Package naming** : le crate `core` a été renommé `controlplane-core` pour éviter le conflit avec la std lib Rust. Le dossier reste `crates/core/`.
- **`@tanstack/react-table` retiré** : trop complexe pour le moment. Ne pas le réintroduire.
- **Proto restructuré** : les fichiers proto sont sous `protocol/france_nuage/resourcemanager/v1/` pour conformité buf lint (PACKAGE_DIRECTORY_MATCH). Respecter cette structure pour tout nouveau .proto.
- **Service proto suffix** : les services proto utilisent le suffixe `Service` (ex: `ResourceManagerService`) pour conformité buf lint. Respecter cette convention.
- **`main.rs` exclu du coverage** : `--ignore-filename-regex 'main\.rs'` — bootstrapping uniquement. Ne pas ajouter de logique dans main.rs.
- **Biome** (pas ESLint) : le linting frontend est via Biome. Ne pas installer ESLint ou Prettier.
- **GitHub Actions** (pas GitLab CI) : le CI utilise GitHub Actions. L'architecture mentionne GitLab mais c'est obsolète.
- **Tonic/Prost 0.14** (pas 0.12/0.13) : l'architecture mentionne tonic 0.12, mais le projet utilise tonic 0.14/prost 0.14. Utiliser les versions du Cargo.toml.
- **Rust edition 2024** : tous les Cargo.toml utilisent `edition = "2024"`.
- **Docker-based CI** : tous les jobs CI utilisent `docker compose run`. Les tests et builds se font dans les containers.

### Fichiers pertinents de Story 1-1

- `docker-compose.yml` — stack dev complète, service `system-tests` avec `profiles: [test]`
- `system-tests/Dockerfile` — installe node-sdk, puis system-tests, puis Playwright chromium
- `system-tests/playwright.config.ts` — config de base (chromium, HTML reporter)
- `system-tests/tests/base.ts` — HomePage POM + fixture Playwright custom
- `keycloak-realm.json` — realm france-nuage, 2 clients (console, controlplane)
- `.github/workflows/ci.yml` — job `system-tests` qui démarre la stack et lance Playwright

## Dev Agent Record

### Agent Model Used

Claude Opus 4.6

### Debug Log References

### Completion Notes List

- **kind remplacé par k3s** : l'utilisateur a explicitement rejeté l'approche kind + scripts shell. k3s est déployé comme service docker-compose natif (profil `k8s`), pas de scripts custom.
- **Anti-pattern identifié** : ne jamais créer de scripts shell custom quand docker-compose suffit. Tout doit être orchestrable via `docker compose up/down`.
- **@kubernetes/client-node** ajouté pour le helper k8s-cleanup.ts. Le kubeconfig est partagé via volume Docker nommé `k3s-kubeconfig`.
- **`Object.assign`** utilisé pour contourner le `readonly server` dans le type `Cluster` de `@kubernetes/client-node`.
- **Pas de Biome** pour system-tests — Biome est configuré uniquement pour `console/`.
- **TypeScript compile** sans erreur (`npx tsc --noEmit` OK).

### File List

**Fichiers CRÉÉS :**
- `system-tests/tests/helpers/accessibility.ts` — helper axe-core WCAG AA
- `system-tests/tests/helpers/auth.ts` — login programmatique Keycloak
- `system-tests/tests/helpers/keycloak-admin.ts` — API admin Keycloak (CRUD users)
- `system-tests/tests/helpers/cleanup.ts` — pattern cleanup (CleanupRegistry)
- `system-tests/tests/helpers/k8s-cleanup.ts` — nettoyage namespaces K8s
- `system-tests/tests/helpers/index.ts` — barrel exports
- `system-tests/tests/pages/base.page.ts` — POM abstrait
- `system-tests/tests/pages/login.page.ts` — POM page login Keycloak
- `system-tests/tests/pages/home.page.ts` — POM page accueil

**Fichiers MODIFIÉS :**
- `system-tests/package.json` — +@axe-core/playwright, +@kubernetes/client-node
- `system-tests/playwright.config.ts` — +projet desktop (1280x720) et tablet (768x1024)
- `system-tests/tests/base.ts` — refacto imports pages/, fixtures enrichies
- `keycloak-realm.json` — +client e2e-tests, +utilisateur test
- `docker-compose.yml` — +service k3s (profil k8s), +volume k3s-kubeconfig, system-tests env/volumes
- `.github/workflows/ci.yml` — +profil k8s dans start/stop services
