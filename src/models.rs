use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: Uuid,
    username: String,
    name: String,
    mail: String,
    created_at: NaiveDateTime,
    modified_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUserPayload {
    username: String,
    name: String,
    mail: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUserPayload {
    username: Option<String>,
    name: Option<String>,
    mail: Option<String>,
}
