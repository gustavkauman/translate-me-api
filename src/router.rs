use crate::api::{user::*, ApiState};
use axum::{
    routing::{delete, get, patch},
    Router,
};
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
        .route("/users", get(get_users).post(create_user))
        .route("/users/:user_id", get(get_user))
        .route("/users/:user_id", patch(update_user))
        .route("/users/:user_id", delete(delete_user))
        .with_state(app_state);

    api
}
