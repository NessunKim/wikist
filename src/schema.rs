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
        created_at -> Timestamp,
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

joinable!(revisions -> actors (actor_id));
joinable!(revisions -> articles (article_id));
joinable!(revisions -> contents (content_id));

allow_tables_to_appear_in_same_query!(
    actors,
    articles,
    authentications,
    contents,
    revisions,
    users,
);
