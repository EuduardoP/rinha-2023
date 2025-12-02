mod models;
mod handlers;
mod docs;
mod utils;
mod db;
mod schema;

use std::{net::SocketAddr, sync::{Arc}};
use tower_http::trace::TraceLayer;
use tracing::debug;
use axum::{Router, routing::{get, post}};

use crate::models::AppState;
use crate::handlers::{get_people_by_id, search_people, count_people, create_people};
use crate::docs::{doc, openapi};
use crate::db::potsgres_repository::PostgresRepository;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    let pool = PostgresRepository::get_pool();
     let port: u16 = std::env::var("PORT")
    .unwrap_or("9999".into())
    .parse()
    .expect("PORT inválida");

    // Run migrations
    {
        let mut conn = pool.get().expect("Failed to get connection for migrations");
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
        debug!("Migrations ran successfully");
    }

    let app_state: AppState = Arc::new(pool);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    let app = Router::new()
        .route("/doc", get(doc))
        .route("/openapi", get(openapi))
        .route("/pessoas", post(create_people))
        .route("/pessoas/{:id}", get(get_people_by_id))
        .route("/pessoas", get(search_people))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    debug!("Server running in http://{}", addr);
    debug!("Docs running in http://{}/doc", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
