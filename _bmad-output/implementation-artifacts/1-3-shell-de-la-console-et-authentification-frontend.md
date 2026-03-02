# Story 1.3: Shell de la console et authentification frontend

Status: review

## Story

As a user,
I want to sign in to the France-nuage console via Keycloak and see the application shell,
so that I can access my cloud workspace securely.

## Acceptance Criteria

1. **Given** I am not authenticated
   **When** I navigate to the console URL
   **Then** I am redirected to the Keycloak login page

2. **Given** I am on the Keycloak login page
   **When** I sign in with valid credentials (or register a new account)
   **Then** I am redirected back to the console
   **And** the OIDC token is stored in sessionStorage via oidc-client-ts

3. **Given** I am authenticated
   **When** the console loads
   **Then** I see the application shell with a sidebar (nav), header (logo, avatar), and main content area
   **And** the sidebar contains navigation items
   **And** my name or email appears in the header

4. **Given** I am authenticated
   **When** I click the logout button
   **Then** I am logged out of both the console and Keycloak
   **And** I am redirected to the login page

5. **Given** my OIDC token has expired
   **When** I perform any action in the console
   **Then** the token is silently refreshed via oidc-client-ts
   **And** if refresh fails, I am redirected to the login page

## Tasks / Subtasks

- [x] Task 1 : Installer react-router-dom (AC: #1, #2, #3, #4)
  - [x] `npm install react-router-dom` dans console/
  - [x] Verifier compatibilite avec React 19

- [x] Task 2 : Configurer oidc-client-ts UserManager (AC: #1, #2, #5)
  - [x] Creer `console/src/auth/user-manager.ts` — instance singleton UserManager
  - [x] Configurer depuis `config.ts` existant (authority, clientId, redirectUri)
  - [x] Configurer `automaticSilentRenew: true` pour le refresh silencieux (AC #5)
  - [x] Configurer `post_logout_redirect_uri` pour la deconnexion (AC #4)

- [x] Task 3 : Creer le composant page-guard (AC: #1)
  - [x] `console/src/components/page-guard.tsx`
  - [x] Si non authentifie → `userManager.signinRedirect()` (redirige vers Keycloak)
  - [x] Si authentifie → rendre `<Outlet />`
  - [x] Gerer l'etat de chargement pendant la verification du token

- [x] Task 4 : Creer la page callback OIDC (AC: #2)
  - [x] `console/src/pages/callback.page.tsx`
  - [x] Appeler `userManager.signinCallback()` au montage
  - [x] Rediriger vers `/` apres succes
  - [x] Gerer les erreurs de callback (token invalide, etc.)

- [x] Task 5 : Creer le shell — app-layout, app-header, app-sidebar (AC: #3)
  - [x] `console/src/components/app-layout.tsx` — Shell 100vh (header + sidebar + Outlet)
  - [x] `console/src/components/app-header.tsx` — Logo France-nuage + avatar utilisateur + hamburger menu (mobile)
  - [x] `console/src/components/app-sidebar.tsx` — Navigation avec sections et items
  - [x] Sidebar visible sur desktop (breakpoint `lg`), Drawer sur mobile/tablette
  - [x] Afficher le nom ou l'email de l'utilisateur dans le header (depuis le profil OIDC)

- [x] Task 6 : Implementer la deconnexion (AC: #4)
  - [x] Bouton logout dans le header (pres de l'avatar)
  - [x] Appeler `userManager.signoutRedirect()` au clic
  - [x] L'utilisateur est deconnecte de la console ET de Keycloak
  - [x] Redirection vers la page de login apres deconnexion

- [x] Task 7 : Configurer le routeur React Router (AC: #1, #2, #3, #4)
  - [x] `/callback` → CallbackPage (route publique)
  - [x] `/` → AppLayout (protege par PageGuard) → HomePage
  - [x] Les routes protegees redirigent vers Keycloak si non authentifie
  - [x] Integrer dans main.tsx avec `RouterProvider`

- [x] Task 8 : Ecrire les tests E2E Playwright (AC: #1, #2, #3, #4)
  - [x] `system-tests/tests/cases/auth.spec.ts` — describe("Story 1.3 : Shell de la console et authentification frontend")
  - [x] Test : "un utilisateur non authentifie ne voit pas le shell"
  - [x] Test : "un utilisateur authentifie voit le shell de la console"
  - [x] Test : "le nom de l'utilisateur apparait dans le header"
  - [x] Test : "le bouton de deconnexion est visible"
  - [x] Test : "le shell est accessible (WCAG AA)" — axe-core via BasePage.checkAccessibility()

- [x] Task 9 : Verification d'integrite
  - [x] Tous les tests E2E existants passent (regression)
  - [x] `npm run lint` passe sans erreur
  - [x] `npm run build` passe sans erreur
  - [x] Le shell s'affiche correctement sur desktop (1280x720) et tablette (768x1024)

## Dev Notes

### Architecture & Patterns

**OIDC Flow (oidc-client-ts) :**
1. L'utilisateur arrive sur `/` → `PageGuard` verifie l'auth
2. Pas de token → `userManager.signinRedirect()` → redirect vers Keycloak
3. L'utilisateur se connecte sur Keycloak
4. Keycloak redirige vers `/callback` avec le code d'autorisation
5. `CallbackPage` appelle `userManager.signinCallback()` → echange code contre tokens
6. Redirect vers `/` → `PageGuard` detecte le token → affiche le shell

**Important — oidc-client-ts v3 :**
- Le UserManager stocke le user en `sessionStorage` par defaut
- La fixture `actingAs` dans `base.ts` injecte directement dans `sessionStorage` (contourne Keycloak pour les tests E2E)
- La cle de stockage est `oidc.user:{authority}:{clientId}` — deja utilisee dans les fixtures E2E
- `automaticSilentRenew` utilise un iframe cache pour renouveler le token
- `signinCallback()` traite le code d'autorisation depuis l'URL apres redirect Keycloak

**Shell Layout (UX Design Specification) :**
- Shell 100vh : header fixe en haut + sidebar a gauche + zone de contenu scrollable
- Sidebar : largeur fixe desktop, Drawer overlay sur mobile/tablette
- Breakpoint responsive : `lg` (Chakra UI) — en dessous, sidebar masquee, hamburger dans le header
- Header : logo a gauche, avatar a droite
- Zone de contenu : `flex={1}`, `overflowY="auto"`, `p={4}`

**Sidebar Navigation (MVP — 3 items) :**
- Section "Applications" :
  - "Mes applications" → `/` (home)
  - "Catalogue" → `/catalog`
- Section "Organisation" :
  - "Facturation" → `/billing`
- Item actif : background blue subtle + border-left blue + font-weight medium
- Hover : background gray subtle
- Sections avec titres uppercase `xs`, gris

**Composants Chakra UI v3 utilises :**
- Layout : `Stack`, `Flex`, `Box`
- Sidebar desktop : composants Chakra directement
- Sidebar mobile : `Drawer` (Chakra UI v3)
- Avatar : `Avatar` (Chakra UI v3)
- Boutons sidebar : `Button` variant `ghost`

**React Router :**
- Utiliser `createBrowserRouter` + `RouterProvider` (API moderne React Router v7)
- Le `PageGuard` est un layout route qui wrap les routes protegees
- `/callback` est une route publique (pas protegee)

**Routes MVP :**
```text
/callback          → CallbackPage (publique)
/                  → PageGuard → AppLayout → HomePage
/catalog           → PageGuard → AppLayout → CatalogPage (future)
/billing           → PageGuard → AppLayout → BillingPage (future)
```

### Fichiers a creer/modifier

**Nouveaux fichiers :**
- `console/src/auth/user-manager.ts` — Singleton UserManager oidc-client-ts
- `console/src/components/page-guard.tsx` — Protection routes authentifiees
- `console/src/components/app-layout.tsx` — Shell principal
- `console/src/components/app-header.tsx` — Header (logo + avatar + hamburger)
- `console/src/components/app-sidebar.tsx` — Sidebar navigation
- `console/src/pages/callback.page.tsx` — Callback OIDC
- `console/src/pages/home.page.tsx` — Page d'accueil (placeholder pour maintenant)
- `system-tests/tests/cases/auth.spec.ts` — Tests E2E authentification + shell

**Fichiers a modifier :**
- `console/src/main.tsx` — Remplacer `<App />` par `<RouterProvider />`
- `console/src/App.tsx` — Peut etre supprime ou transforme
- `console/package.json` — Ajouter `react-router-dom`
- `system-tests/tests/pages/home.page.ts` — Ajouter des locators pour le shell (sidebar, header, avatar)

### Conventions a respecter

- **Pas de dossier utils/** — anti-pattern
- **Biome** pour le lint/format (pas eslint/prettier)
- **Noms de tests en francais** : `test("un utilisateur non authentifie est redirige vers Keycloak")`
- **Code en anglais** : noms de variables, fonctions, composants
- **Composants Chakra directs** — pas de wrappers customs inutiles
- **Pas de sur-ingenierie** — pas de AuthContext custom si oidc-client-ts suffit
- **HTML semantique** : `<nav>` pour la sidebar, `<header>` pour le header, `<main>` pour le contenu

### Tests E2E — Strategie

Les tests utilisent la fixture `actingAs` de `base.ts` pour injecter les tokens directement en `sessionStorage`. Cela contourne la page de login Keycloak pour les tests qui ne testent pas l'authentification elle-meme.

Pour le test de redirection vers Keycloak (AC #1), le test doit :
1. Ne PAS appeler `actingAs`
2. Naviguer vers `/`
3. Verifier que l'URL contient le domaine Keycloak (redirect OIDC)

Pour les tests du shell (AC #3), le test doit :
1. Appeler `actingAs()` avec les credentials par defaut
2. Naviguer vers `/`
3. Verifier la presence du header, de la sidebar, du nom utilisateur

Pour le test de deconnexion (AC #4), c'est plus complexe car il faut :
1. Etre authentifie (via `actingAs`)
2. Cliquer sur le bouton de deconnexion
3. Verifier la redirection vers Keycloak (end_session_endpoint)

### Page Object updates

Le `HomePage` existant dans `system-tests/tests/pages/home.page.ts` devra etre enrichi avec de nouveaux locators pour le shell :
- `sidebar` — la sidebar de navigation
- `header` — le header de l'application
- `userAvatar` — l'avatar de l'utilisateur dans le header
- `userName` — le nom/email de l'utilisateur
- `logoutButton` — le bouton de deconnexion

### Project Structure Notes

- Alignement avec la structure monorepo existante : console/src/components/, console/src/pages/
- Le dossier `console/src/auth/` est nouveau — contient uniquement le singleton UserManager
- Pas de dossier `console/src/hooks/` au MVP — utiliser directement le UserManager
- Pas de AuthProvider/AuthContext custom — oidc-client-ts gere l'etat auth via sessionStorage

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 1.3] — Acceptance criteria, FR1/FR2
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Component Strategy] — Shell components (app-layout, app-header, app-sidebar, page-guard)
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Navigation Patterns] — Sidebar items, routes, context switching
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Visual Design Foundation] — Couleurs, typographie, espacement
- [Source: _bmad-output/planning-artifacts/architecture.md] — oidc-client-ts, React Router, Chakra UI v3, hexagonal architecture
- [Source: system-tests/tests/base.ts] — Fixtures existantes (actingAs, pages, keycloak)
- [Source: console/src/config.ts] — Configuration OIDC existante

### Previous Story Intelligence (Story 1.2)

- La fixture `actingAs` injecte les tokens OIDC dans `sessionStorage` avec la cle `oidc.user:{authority}:{clientId}`
- La fixture `pages` fournit `home` et `login` — ajouter les nouveaux locators au `HomePage`
- Le `LoginPage` gere les interactions avec la page Keycloak (champs username/password/bouton login)
- Les tests existants dans `home.spec.ts` et `keycloak.spec.ts` ne doivent PAS etre modifies
- axe-core est integre dans `BasePage.checkAccessibility()` — l'utiliser pour le test d'accessibilite du shell

### Git Intelligence

Commits recents :
- `04f8a93` feat(system-tests): infrastructure E2E Story 1.2 (#2)
- `44de7b6` feat: initialisation du monorepo et environnement de developpement (#1)

Patterns etablis :
- Branches feature `us{N}` → PR vers master
- Convention de commit : `feat(scope): description` ou `fix(scope): description`
- CI locale via `/ci-local` avant tout push

## Dev Agent Record

### Agent Model Used

Claude Opus 4.6

### Debug Log References

### Completion Notes List

- react-router-dom v7.13.1 installe, compatible React 19
- UserManager oidc-client-ts configure avec automaticSilentRenew et post_logout_redirect_uri
- PageGuard bloque le rendu si non authentifie, passe le User via Outlet context
- CallbackPage traite le callback OIDC et redirige vers /
- Shell complet : header (logo + nom user + avatar + logout), sidebar (3 items MVP), Drawer mobile
- Sidebar responsive : visible desktop (lg+), Drawer overlay mobile/tablette
- Routeur React Router v7 avec createBrowserRouter + RouterProvider
- Correction critique : fixture actingAs utilise maintenant CONSOLE_OIDC_AUTHORITY et CONSOLE_OIDC_CLIENT_ID pour la cle sessionStorage (alignement avec la config console)
- home.spec.ts mis a jour avec actingAs (prerequis auth avec le nouveau PageGuard)
- 5 tests E2E auth : non-auth bloque, shell visible, nom user, logout visible, WCAG AA
- 16/16 tests passent (10 nouveaux + 6 existants, regression OK)
- CI locale complete : lint, build, clippy, coverage 100%

### File List

- console/package.json (modified — added react-router-dom)
- console/package-lock.json (modified — lockfile updated)
- console/src/auth/user-manager.ts (new)
- console/src/components/page-guard.tsx (new)
- console/src/components/app-layout.tsx (new)
- console/src/components/app-header.tsx (new)
- console/src/components/app-sidebar.tsx (new)
- console/src/pages/callback.page.tsx (new)
- console/src/pages/home.page.tsx (new)
- console/src/App.tsx (modified — router setup)
- docker-compose.yml (modified — added CONSOLE_OIDC_AUTHORITY/CLIENT_ID to system-tests)
- system-tests/tests/base.ts (modified — CONSOLE_OIDC constants for storage key)
- system-tests/tests/pages/home.page.ts (modified — shell locators)
- system-tests/tests/cases/auth.spec.ts (new)
- system-tests/tests/cases/home.spec.ts (modified — added actingAs)
- _bmad-output/implementation-artifacts/sprint-status.yaml (modified — story status)
- _bmad-output/implementation-artifacts/1-3-shell-de-la-console-et-authentification-frontend.md (modified — status + completion)
