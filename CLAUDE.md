# France-nuage

## Langue
Communication en francais.

## Git workflow
- **Avant tout commit+push, lancer les checks CI en local via Docker** (skill `/ci-local`)
- Les commandes sont identiques a `.github/workflows/ci.yml` (docker compose run)
- Ne jamais pousser sans avoir valide localement : lint, build, clippy, coverage, E2E

## Code
- Rust : 100% code coverage (cargo-llvm-cov, `--fail-under-lines 100`)
- Frontend : Biome (pas eslint/prettier), pas de tests unitaires, E2E Playwright uniquement
- Pas de dossier utils/ (anti-pattern), pas de sur-ingenierie
- Noms de tests en francais
