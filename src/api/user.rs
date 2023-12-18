use crate::api::{ApiError, ApiState};
use crate::schema::users::{self, dsl::*};
use axum::routing::{delete, get, patch, post};
use axum::Router;
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use diesel::result::Error as DieselError;
use diesel::{prelude::*, Insertable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: Uuid,
    username: String,
    name: String,
    mail: String,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
struct CreateUserPayload {
    username: String,
    name: String,
    mail: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
struct UpdateUserPayload {
    username: Option<String>,
    name: Option<String>,
    mail: Option<String>,
}

pub fn create_user_api_router() -> Router<ApiState> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:user_id", get(get_user))
        .route("/users/:user_id", patch(update_user))
        .route("/users/:user_id", delete(delete_user))
}

async fn get_user(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, ApiError> {
    let mut conn = state.db_conn_pool.get().await?;

    let user = users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .map_err(|db_error| match db_error {
            DieselError::NotFound => ApiError::NotFound(user_id),
            _ => ApiError::InternalServerError,
        })?;

    Ok(Json(user))
}

async fn get_users(
    State(state): State<ApiState>,
) -> Result<Json<Vec<User>>, ApiError> {
    let mut conn = state.db_conn_pool.get().await?;

    let results: Vec<User> = users
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(results))
}

async fn create_user(
    State(state): State<ApiState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<User>, ApiError> {
    let mut conn = state.db_conn_pool.get().await?;

    // TODO: Add checks for constraint violations
    let new_user = diesel::insert_into(users::table)
        .values(payload)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(new_user))
}

async fn update_user(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<User>, ApiError> {
    let mut conn = state.db_conn_pool.get().await?;

    let updated_user = diesel::update(users::table)
        .filter(id.eq(user_id))
        .set(payload)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|db_err| match db_err {
            DieselError::NotFound => ApiError::BadRequest(format!(
                "User with id {} not found",
                user_id
            )),
            _ => ApiError::InternalServerError,
        })?;

    Ok(Json(updated_user))
}

async fn delete_user(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<(), ApiError> {
    let mut conn = state.db_conn_pool.get().await?;

    let _ = diesel::delete(users::table)
        .filter(id.eq(user_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(())
}
