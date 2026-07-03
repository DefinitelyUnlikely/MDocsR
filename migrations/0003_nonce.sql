CREATE TABLE IF NOT EXISTS registration_nonces (
    nonce VARCHAR(255) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
                                               );

