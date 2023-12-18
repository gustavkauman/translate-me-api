use crate::api::{ApiError, ApiState};
use crate::schema::workspaces::{self, dsl::*};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, PartialEq)]
#[diesel(table_name = crate::schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workspace {
    id: Uuid,
    name: String,
    created_by: Uuid,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::workspaces)]
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
    let mut conn = state.db_conn_pool.get().await?;

    let workspaces_array = workspaces
        .select(Workspace::as_select())
        .load(&mut conn)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(workspaces_array))
}

async fn get_workspace(
    State(state): State<ApiState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Workspace>, ApiError> {
    let mut conn = state.db_conn_pool.get().await?;

    let workspace = workspaces
        .filter(id.eq(workspace_id))
        .select(Workspace::as_select())
        .first(&mut conn)
        .await
        .map_err(|db_error| match db_error {
            DieselError::NotFound => ApiError::NotFound(workspace_id),
            _ => ApiError::InternalServerError,
        })?;

    Ok(Json(workspace))
}

async fn create_workspace(
    State(state): State<ApiState>,
    Json(payload): Json<CreateWorkspacePayload>,
) -> Result<Json<Workspace>, ApiError> {
    let mut conn = state.db_conn_pool.get().await?;

    let new_workspace = diesel::insert_into(workspaces::table)
        .values(payload)
        .returning(Workspace::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(new_workspace))
}
