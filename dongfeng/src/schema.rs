// @generated automatically by Diesel CLI.

diesel::table! {
    dregs (id) {
        id -> Integer,
        username -> Text,
        user_id -> Integer,
        count -> Integer,
        paid -> Bool,
        taken_time -> Date,
        production_time -> Date,
        amount -> Integer,
        price -> Integer,
        description -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        description -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dregs,
    posts,
);
