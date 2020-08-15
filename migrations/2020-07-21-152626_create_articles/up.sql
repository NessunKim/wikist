-- Your SQL goes here
CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(300) NOT NULL,
    wikitext TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
)