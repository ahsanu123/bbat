// @generated automatically by Diesel CLI.

diesel::table! {
    product (id) {
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
