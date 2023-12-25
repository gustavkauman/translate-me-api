use crate::api::{ApiError, ApiState};
use axum::routing::{delete, get, patch, post};
use axum::Router;
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, PartialEq, sqlx::FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    name: String,
    mail: String,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct CreateUserPayload {
    username: String,
    name: String,
    mail: String,
}

#[derive(Deserialize)]
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
    let user = sqlx::query_as::<_, User>("select * from users where id = $1")
        .bind(user_id)
        .fetch_one(&state.db_conn_pool)
        .await
        // TODO: Properly map error when user does not exist
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(user))
}

async fn get_users(
    State(state): State<ApiState>,
) -> Result<Json<Vec<User>>, ApiError> {
    let results: Vec<User> = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&state.db_conn_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(results))
}

async fn create_user(
    State(state): State<ApiState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<User>, ApiError> {
    let new_user = sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
            with inserted_user as (
                insert into users (username, name, mail)
                values ($1, $2, $3)
                returning id, username, name, mail, created_at, modified_at
            )
            select * from
            inserted_user
        "#,
        payload.username,
        payload.name,
        payload.mail
    )
    .fetch_one(&state.db_conn_pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db_err) => match db_err.constraint() {
            Some(_) => ApiError::BadRequest(String::from(
                "Username and/or e-mail is already taken",
            )),
            None => ApiError::InternalServerError,
        },
        _ => ApiError::InternalServerError,
    })?;

    Ok(Json(new_user))
}

async fn update_user(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<User>, ApiError> {
    let existing_data =
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&state.db_conn_pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => ApiError::NotFound(user_id),
                _ => ApiError::InternalServerError,
            })?;

    let updated_user = sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
            update users set username = $1, name = $2, mail = $3 where id = $4 returning *
        "#,
        payload.username.unwrap_or(existing_data.username),
        payload.name.unwrap_or(existing_data.name),
        payload.mail.unwrap_or(existing_data.mail),
        user_id
    )
    .fetch_one(&state.db_conn_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(updated_user))
}

async fn delete_user(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<(), ApiError> {
    let res = sqlx::query!("delete from users where id = $1", user_id)
        .execute(&state.db_conn_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    if res.rows_affected() == 0 {
        return Err(ApiError::NotFound(user_id));
    }

    Ok(())
}
