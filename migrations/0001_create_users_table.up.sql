CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    wallet_address VARCHAR(255) UNIQUE,
    twitter_id VARCHAR(255) NOT NULL UNIQUE,
    encrypted_password VARCHAR(255) NOT NULL,
    referral_code INTEGER NOT NULL UNIQUE,
    total_points INTEGER DEFAULT 0,
    finished_tasks INTEGER[] DEFAULT '{}',
    referral_points INTEGER DEFAULT 0,
    referred_by INTEGER[] DEFAULT '{}',
    referrer_id INTEGER
);
