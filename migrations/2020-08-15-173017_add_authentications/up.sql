-- Your SQL goes here
CREATE TABLE authentications(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER NOT NULL,
    provider VARCHAR(255) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
    UNIQUE(user_id, provider)
)