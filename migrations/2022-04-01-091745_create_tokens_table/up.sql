CREATE TABLE IF NOT EXISTS tokens (
    hash text PRIMARY KEY,
    user_id text NOT NULL REFERENCES submitters ON DELETE CASCADE,
    expiry timestamp(0) with time zone NOT NULL,
    scope text NOT NULL
);
