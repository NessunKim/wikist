-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
CREATE TABLE actors (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER UNIQUE NULL,
    ip_address CIDR UNIQUE NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);
CREATE TABLE authentications(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER NOT NULL,
    provider VARCHAR(255) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
    UNIQUE(user_id, provider)
);
CREATE TABLE articles (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title VARCHAR(300) NOT NULL,
    latest_revision_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
CREATE TABLE contents (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    wikitext TEXT NOT NULL
);
CREATE TABLE revisions (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    article_id INTEGER NOT NULL,
    actor_id INTEGER NOT NULL,
    content_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    CONSTRAINT fk_article FOREIGN KEY(article_id) REFERENCES articles(id),
    CONSTRAINT fk_actor FOREIGN KEY(actor_id) REFERENCES actors(id),
    CONSTRAINT fk_content FOREIGN KEY(content_id) REFERENCES contents(id)
);