// @generated automatically by Diesel CLI.

diesel::table! {
    posts (post_id) {
        post_id -> Int4,
        title -> Varchar,
        body -> Text,
        is_published -> Bool,
    }
}
