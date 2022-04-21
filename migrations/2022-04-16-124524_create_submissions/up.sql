CREATE TABLE IF NOT EXISTS submission_requests (
    id bigserial PRIMARY KEY,
    user_id text NOT NULL REFERENCES submitters ON DELETE CASCADE,
    compounds text[] NOT NULL,
    accession text NOT NULL,
    start_nt int,
    end_nt int
);

CREATE SEQUENCE IF NOT EXISTS entry_number AS BIGINT INCREMENT BY 1 START WITH 2500;