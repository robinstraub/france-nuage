Execute les memes checks que la CI GitHub Actions, en local via Docker, avant de commit et push.

Lance les etapes suivantes dans l'ordre. Arrete-toi a la premiere erreur, corrige, puis relance.

## 1. Checks rapides (parallelisables)

Lance en parallele :
- `docker compose run --rm --no-deps console npm run lint`
- `docker compose run --rm --no-deps --workdir /app controlplane buf lint protocol/`

## 2. Build controlplane + console

Lance en parallele :
- `docker compose run --rm --no-deps controlplane cargo build --workspace`
- `docker compose run --rm --no-deps console npm run build`

## 3. Checks post-build (parallelisables)

Lance en parallele :
- `docker compose run --rm --no-deps controlplane cargo clippy --workspace -- -D warnings`
- `docker compose run --rm --no-deps controlplane cargo llvm-cov --workspace --ignore-filename-regex 'main\.rs' --fail-under-lines 100`

## 4. Tests E2E

- `docker compose up -d --build --wait console controlplane keycloak postgres spicedb spicedb-migrate traefik`
- `docker compose run -v $(pwd)/playwright-report:/app/playwright-report system-tests`
- `docker compose down`

## Resultat

Si tout passe : commit et push.
Si un check echoue : corrige le probleme, relance uniquement les etapes echouees, puis commit et push.
