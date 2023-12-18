use crate::api::{ApiError, ApiState};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, PartialEq, sqlx::FromRow)]
pub struct Workspace {
    id: Uuid,
    name: String,
    created_by: Uuid,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct CreateWorkspacePayload {
    name: String,
    created_by: Uuid, // TODO: This should be obtained from token instead
}

pub fn create_workspace_api_router() -> Router<ApiState> {
    Router::new()
        .route("/workspaces", get(get_all_workspaces))
        .route("/workspaces", post(create_workspace))
        .route("/workspaces/:workspace_id", get(get_workspace))
}

async fn get_all_workspaces(
    State(state): State<ApiState>,
) -> Result<Json<Vec<Workspace>>, ApiError> {
    let workspaces_array = sqlx::query_as!(
        Workspace,
        // language=PostgreSQL
        r#"
            select * from workspaces
        "#
    )
    .fetch_all(&state.db_conn_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(workspaces_array))
}

async fn get_workspace(
    State(state): State<ApiState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Workspace>, ApiError> {
    let workspace = sqlx::query_as!(
        Workspace,
        // language=PostgreSQL
        r#"
            select * from workspaces where id = $1
        "#,
        workspace_id
    )
    .fetch_one(&state.db_conn_pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => ApiError::NotFound(workspace_id),
        _ => ApiError::InternalServerError,
    })?;

    Ok(Json(workspace))
}

async fn create_workspace(
    State(state): State<ApiState>,
    Json(payload): Json<CreateWorkspacePayload>,
) -> Result<Json<Workspace>, ApiError> {
    let new_workspace = sqlx::query_as!(
        Workspace,
        // language=PostgreSQL
        r#"
            insert into workspaces (name, created_by)
            values ($1, $2)
            returning *
        "#,
        payload.name,
        payload.created_by
    )
    .fetch_one(&state.db_conn_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(new_workspace))
}
