// @generated automatically by Diesel CLI.

diesel::table! {
    members (id) {
        id -> Varchar,
        username -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Varchar,
        author_id -> Varchar,
        title -> Text,
        content -> Text,
        kind -> Int4,
    }
}

diesel::joinable!(posts -> members (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    members,
    posts,
);
