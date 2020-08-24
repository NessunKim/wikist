table! {
    actors (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        ip_address -> Nullable<Cidr>,
    }
}

table! {
    articles (id) {
        id -> Int4,
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
joinable!(authentications -> users (user_id));
joinable!(revisions -> actors (actor_id));
joinable!(revisions -> articles (article_id));
joinable!(revisions -> contents (content_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    actors,
    articles,
    authentications,
    contents,
    revisions,
    roles,
    user_roles,
    users,
);
