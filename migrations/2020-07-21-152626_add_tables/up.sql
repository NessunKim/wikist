-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE roles (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(50) UNIQUE NOT NULL
);
CREATE TABLE user_roles (
    user_id INTEGER NOT NULL REFERENCES users,
    role_id INTEGER NOT NULL REFERENCES roles,
    CONSTRAINT user_roles_pkey PRIMARY KEY (user_id, role_id)
);
CREATE TABLE actors (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER UNIQUE NULL REFERENCES users,
    ip_address CIDR UNIQUE NULL
);
CREATE TABLE authentications(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER NOT NULL REFERENCES users,
    provider VARCHAR(255) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, provider)
);
CREATE TABLE namespaces (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(30) UNIQUE NOT NULL
);
CREATE TABLE articles (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    namespace_id INTEGER NOT NULL REFERENCES namespaces,
    title VARCHAR(300) NOT NULL,
    latest_revision_id INTEGER NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(namespace_id, title)
);
CREATE TABLE article_searches (
    article_id INTEGER PRIMARY KEY REFERENCES articles,
    vector TSVECTOR NOT NULL
);
CREATE INDEX article_searches_vector_idx ON article_searches USING GIN (vector);
CREATE TABLE contents (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    wikitext TEXT NOT NULL
);
CREATE TABLE revisions (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    article_id INTEGER NOT NULL REFERENCES articles,
    actor_id INTEGER NOT NULL REFERENCES actors,
    content_id INTEGER NOT NULL REFERENCES contents,
    comment TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE redirections (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    namespace_id INTEGER NOT NULL REFERENCES namespaces,
    title VARCHAR(300) NOT NULL,
    target_id INTEGER NOT NULL REFERENCES articles,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(namespace_id, title)
);
CREATE TABLE namespace_permissions (
    namespace_id INTEGER NOT NULL REFERENCES namespaces,
    role_id INTEGER NOT NULL REFERENCES roles,
    can_create BOOLEAN NOT NULL,
    can_read BOOLEAN NOT NULL,
    can_edit BOOLEAN NOT NULL,
    can_rename BOOLEAN NOT NULL,
    can_delete BOOLEAN NOT NULL,
    can_grant BOOLEAN NOT NULL,
    CONSTRAINT namespace_permissions_pkey PRIMARY KEY (namespace_id, role_id)
);
CREATE TABLE article_permissions (
    article_id INTEGER NOT NULL REFERENCES articles,
    role_id INTEGER NOT NULL REFERENCES roles,
    can_read BOOLEAN NULL,
    can_edit BOOLEAN NULL,
    can_rename BOOLEAN NULL,
    can_delete BOOLEAN NULL,
    CONSTRAINT article_permissions_pkey PRIMARY KEY (article_id, role_id)
);
INSERT INTO namespaces(name)
VALUES ('_DEFAULT');
INSERT INTO roles(name)
VALUES ('Root');
INSERT INTO roles(name)
VALUES ('Anonymous');
INSERT INTO roles(name)
VALUES ('LoggedIn');
INSERT INTO namespace_permissions (
        namespace_id,
        role_id,
        can_create,
        can_read,
        can_edit,
        can_rename,
        can_delete,
        can_grant
    )
VALUES (1, 1, true, true, true, true, true, true),
    (1, 2, true, true, true, false, false, false),
    (1, 3, true, true, true, true, false, false);