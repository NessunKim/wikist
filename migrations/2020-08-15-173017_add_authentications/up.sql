-- Your SQL goes here
CREATE TABLE authentications(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER,
    provider VARCHAR(255) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    CONSTRAINT fk_customer FOREIGN KEY(user_id) REFERENCES users(id)
)