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

diesel::table! {
    workspaces (id) {
        id -> Uuid,
        #[max_length = 250]
        name -> Varchar,
        created_by -> Uuid,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
    }
}

diesel::joinable!(workspaces -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    workspaces,
);
