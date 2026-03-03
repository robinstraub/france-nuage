CREATE TABLE users (
    id UUID PRIMARY KEY,
    keycloak_id TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL,
    name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_keycloak_id ON users (keycloak_id);
