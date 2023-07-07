CREATE TABLE IF NOT EXISTS clients (
    id TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS tokens (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL REFERENCES clients (id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);