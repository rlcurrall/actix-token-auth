CREATE TABLE personal_access_tokens (
    id              BIGSERIAL       NOT NULL PRIMARY KEY,
    user_id         BIGINT          NOT NULL REFERENCES users(id),
    name            VARCHAR(255)    NOT NULL,
    token           VARCHAR(128)     NOT NULL,
    abilities       TEXT[]          NULL,
    last_used_at    TIMESTAMPTZ(0)  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at      TIMESTAMPTZ(0)  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMPTZ(0)  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMPTZ(0)  NULL
);

-- index (user_id)
CREATE INDEX idx_personal_access_tokens_user_id
    ON personal_access_tokens (user_id);

-- index (token)
CREATE UNIQUE INDEX idx_personal_access_tokens_token
    ON personal_access_tokens (token ASC)
    WHERE deleted_at IS NULL;

-- trigger (updated_at)
CREATE TRIGGER tg_personal_access_tokens_updated_at
    BEFORE UPDATE
    ON personal_access_tokens
    FOR EACH ROW
    EXECUTE PROCEDURE track_row_updated();
