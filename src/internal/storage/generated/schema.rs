// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
