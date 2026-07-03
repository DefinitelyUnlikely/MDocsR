CREATE TABLE IF NOT EXISTS user_passkeys (
     id VARCHAR(255) PRIMARY KEY, -- Credential ID (hex-encoded)
     user_id VARCHAR(255) NOT NULL REFERENCES users (id) ON DELETE CASCADE,
     name VARCHAR(255) NOT NULL,
     credential_id BYTEA NOT NULL UNIQUE,
     passkey JSONB NOT NULL, -- Serialized Passkey struct from webauthn-rs
     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_user_passkeys_user_id ON user_passkeys (user_id);