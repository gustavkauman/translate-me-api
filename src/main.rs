use axum::extract::{Path, State};
use axum::Json;
use axum::{routing::get, Router};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use translate_me::models::*;
use translate_me::schema::users::{self, dsl::*};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_config =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(env::var("DATABASE_URL").unwrap());
    let db_connection_pool = Pool::builder(db_config).build().unwrap();

    let app_state = AppState {
        pool: db_connection_pool,
    };

    let api = Router::new()
        .route("/", get(|| async { "Hello, API!" }))
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:user_id",
            get(get_user).patch(update_user).delete(delete_user),
        )
        .with_state(app_state);

    let app = Router::new().nest("/api", api);

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_user(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> Json<User> {
    let mut conn = state.pool.get().await.unwrap();

    let user = users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .unwrap();

    Json(user)
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let mut conn = state.pool.get().await.unwrap();

    let results: Vec<User> = users
        .select(User::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    Json(results)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Json<User> {
    let mut conn = state.pool.get().await.unwrap();

    let new_user = insert_into(users::table)
        .values(payload)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    Json(new_user)
}

async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Json<User> {
    let mut conn = state.pool.get().await.unwrap();

    let user = users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .unwrap();

    let updated_user = update(&user)
        .set(payload)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    Json(updated_user)
}

async fn delete_user(State(state): State<AppState>, Path(user_id): Path<Uuid>) {
    let mut conn = state.pool.get().await.unwrap();

    let user = users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .unwrap();

    let _ = delete(&user).execute(&mut conn).await.unwrap();
}
