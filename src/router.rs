use crate::api::{
    user::create_user_api_router, workspace::create_workspace_api_router,
    ApiState,
};
use axum::Router;
use sqlx::postgres::PgPoolOptions;

pub async fn create_api_router() -> Router {
    let db_connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app_state = ApiState {
        db_conn_pool: db_connection_pool,
    };

    let api = Router::new()
        .merge(create_user_api_router())
        .merge(create_workspace_api_router())
        .with_state(app_state);

    api
}
