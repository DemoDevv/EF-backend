// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        pseudo -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Text,
        google_id -> Nullable<Text>,
        password -> Nullable<Text>,
        created_at -> Timestamp,
    }
}
