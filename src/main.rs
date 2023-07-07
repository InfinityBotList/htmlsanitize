use axum::{routing::post, Router, http};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::{CorsLayer, Any}, catch_panic::CatchPanicLayer};
use std::{net::SocketAddr, sync::Arc};

mod sanitizer;
mod query;
mod config;
mod types;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    const MAX_CONNECTIONS: u32 = 3; // max connections to the database, we don't need too many here

    std::env::set_var("RUST_LOG", "htmlsanitize=info");

    env_logger::init();

    let state = Arc::new(types::AppState {
        pool: PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&config::CONFIG.database_url)
            .await
            .expect("Could not initialize connection")
    });

    // build our application with a route
    let app = Router::new()
    .route("/", post(sanitize_handler))
    .route("/query", post(query::query))
    .with_state(state)
    .layer(CatchPanicLayer::new())
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT, http::header::HeaderName::from_static("x-client")]),
    );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 5810));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn sanitize_handler(
    body: String,
) -> String {
    sanitizer::sanitize(&body)
}