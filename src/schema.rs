// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Integer,
        description -> Text,
        delay -> Integer,
        state -> Text,
    }
}
