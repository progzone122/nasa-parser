// @generated automatically by Diesel CLI.

diesel::table! {
    news (id) {
        id -> Integer,
        #[max_length = 500]
        title -> Varchar,
        #[max_length = 500]
        short_description -> Nullable<Varchar>,
        #[max_length = 500]
        image -> Nullable<Varchar>,
        #[max_length = 200]
        url -> Varchar,
        post_id -> Integer,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        html -> Text,
        text -> Text,
    }
}

diesel::joinable!(news -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    news,
    posts,
);
