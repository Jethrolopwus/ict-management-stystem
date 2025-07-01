// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
