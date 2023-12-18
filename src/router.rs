use crate::api::{user::*, workspace::create_workspace_api_router, ApiState};
use axum::Router;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

pub fn create_api_router() -> Router {
    let db_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        std::env::var("DATABASE_URL").unwrap(),
    );
    let db_connection_pool = Pool::builder(db_config).build().unwrap();

    let app_state = ApiState {
        db_conn_pool: db_connection_pool,
    };

    let api = Router::new()
        .merge(create_user_api_router())
        .merge(create_workspace_api_router())
        .with_state(app_state);

    api
}
