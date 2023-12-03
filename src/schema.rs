// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 250]
        name -> Varchar,
        #[max_length = 250]
        mail -> Varchar,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
    }
}
