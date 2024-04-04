// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        password_hash -> Text,
        salt -> Text,
        full_name -> Text,
        created_time -> Timestamp,
    }
}
