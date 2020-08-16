table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        wikitext -> Text,
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
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(authentications -> users (user_id));

allow_tables_to_appear_in_same_query!(
    articles,
    authentications,
    users,
);
