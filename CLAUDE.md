# France-nuage

## Langue
Communication en francais.

## Environnement de developpement — DOCKER OBLIGATOIRE
- **JAMAIS de `cargo`, `npm`, `npx` directement sur la machine hote**
- Toujours utiliser `docker compose run` pour executer les commandes :
  - `docker compose run --rm controlplane cargo build`
  - `docker compose run --rm controlplane cargo test`
  - `docker compose run --rm controlplane cargo clippy`
  - `docker compose run --rm console npm run build`
  - `docker compose run --rm console npm run lint`
- Les volumes Docker montent le code local (`./controlplane:/app/controlplane`, `./protocol:/app/protocol`)
- Pas besoin de `docker compose build` a chaque modification (les volumes sont montes)
- Le premier `docker compose build` est necessaire pour creer l'image (installe protoc, cargo-llvm-cov, etc.)

## Git workflow
- **Avant tout commit+push, lancer les checks CI en local via Docker** (skill `/ci-local`)
- Les commandes sont identiques a `.github/workflows/ci.yml` (docker compose run)
- Ne jamais pousser sans avoir valide localement : lint, build, clippy, coverage, E2E

## Code
- Rust : 100% code coverage (cargo-llvm-cov, `--fail-under-lines 100`)
- Frontend : Biome (pas eslint/prettier), pas de tests unitaires, E2E Playwright uniquement
- Pas de dossier utils/ (anti-pattern), pas de sur-ingenierie
- Noms de tests en francais
