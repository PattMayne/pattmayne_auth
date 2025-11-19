-- 0001_init.sql

-- I'm actually using MariaDB which is supposedly a drop-in replacement for MySQL


CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(255) NOT NULL DEFAULT 'player',
    created_timestamp TIMESTAMP NOT NULL DEFAULT UTC_TIMESTAMP,
    email_verified BOOL NOT NULL DEFAULT FALSE
);


-- refresh tokens to get new JWTs
CREATE TABLE IF NOT EXISTS refresh_tokens (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    client_id INT NOT NULL,
    token VARCHAR(255) NOT NULL UNIQUE,
    created_timestamp TIMESTAMP DEFAULT UTC_TIMESTAMP,
    expires_timestamp TIMESTAMP NOT NULL
);


CREATE TABLE IF NOT EXISTS client_sites (
    id INT AUTO_INCREMENT PRIMARY KEY,
    client_id VARCHAR(100) NOT NULL UNIQUE, -- public identifier. random string
    client_secret VARCHAR(255), -- only for confidential clients (ie backend, not user)
    name VARCHAR(100) NOT NULL,
    domain VARCHAR(255) NOT NULL UNIQUE,
    redirect_uri VARCHAR(255) NOT NULL, -- maybe not needed. Keeping for future-proofing
    description TEXT,
    logo_url VARCHAR(255),
    is_active BOOL DEFAULT TRUE,
    scopes VARCHAR(255), -- maybe not needed. Keeping for future-proofing
    type VARCHAR(50), -- e.g. "confidential" (default), "public", "native" (mobile/desktop)
    is_internal BOOL NOT NULL DEFAULT FALSE, -- only "TRUE" for self (auth site, this site)
    created_timestamp TIMESTAMP DEFAULT UTC_TIMESTAMP
);
