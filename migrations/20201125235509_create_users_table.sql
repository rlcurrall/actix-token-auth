-- table
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    full_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(0)
);

-- index (deleted_at)
CREATE INDEX idx_users_deleted_at
  ON users (deleted_at ASC);

-- index (email)
CREATE UNIQUE INDEX idx_users_email
  ON users (email ASC)
  WHERE deleted_at IS NULL;

-- trigger (updated_at)
CREATE TRIGGER tg_users_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
    EXECUTE PROCEDURE track_row_updated();
