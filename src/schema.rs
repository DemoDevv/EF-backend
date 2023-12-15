// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        salt -> Text,
        created_at -> Timestamp,
    }
}
