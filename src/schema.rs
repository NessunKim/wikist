table! {
    actors (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        ip_address -> Nullable<Cidr>,
    }
}

table! {
    article_permissions (article_id, role_id) {
        article_id -> Int4,
        role_id -> Int4,
        can_read -> Nullable<Bool>,
        can_edit -> Nullable<Bool>,
        can_rename -> Nullable<Bool>,
        can_delete -> Nullable<Bool>,
    }
}

table! {
    articles (id) {
        id -> Int4,
        namespace_id -> Int4,
        title -> Varchar,
        latest_revision_id -> Int4,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    authentications (id) {
        id -> Int4,
        user_id -> Int4,
        provider -> Varchar,
        provider_user_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    contents (id) {
        id -> Int4,
        wikitext -> Text,
    }
}

table! {
    namespace_permissions (namespace_id, role_id) {
        namespace_id -> Int4,
        role_id -> Int4,
        can_create -> Bool,
        can_read -> Bool,
        can_edit -> Bool,
        can_rename -> Bool,
        can_delete -> Bool,
        can_grant -> Bool,
    }
}

table! {
    namespaces (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    redirections (id) {
        id -> Int4,
        namespace_id -> Int4,
        title -> Varchar,
        target_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    revisions (id) {
        id -> Int4,
        article_id -> Int4,
        actor_id -> Int4,
        content_id -> Int4,
        comment -> Text,
        created_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    user_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(actors -> users (user_id));
joinable!(article_permissions -> articles (article_id));
joinable!(article_permissions -> roles (role_id));
joinable!(articles -> namespaces (namespace_id));
joinable!(authentications -> users (user_id));
joinable!(namespace_permissions -> namespaces (namespace_id));
joinable!(namespace_permissions -> roles (role_id));
joinable!(redirections -> articles (target_id));
joinable!(redirections -> namespaces (namespace_id));
joinable!(revisions -> actors (actor_id));
joinable!(revisions -> articles (article_id));
joinable!(revisions -> contents (content_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    actors,
    article_permissions,
    articles,
    authentications,
    contents,
    namespace_permissions,
    namespaces,
    redirections,
    revisions,
    roles,
    user_roles,
    users,
);
