use axum::{routing::{post, get}, Router, http::{self, HeaderMap, StatusCode}, response::IntoResponse};
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

    use utoipa::OpenApi;
    #[derive(OpenApi)]
    #[openapi(paths(query::query), components(schemas(query::ServerResponse, query::ServerError, query::Query)))]
    struct ApiDoc;

    async fn docs() -> impl IntoResponse {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let data = ApiDoc::openapi().to_json();

        if let Ok(data) = data {
            return (headers, data).into_response();
        }

        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate docs".to_string()).into_response()
    }

    // build our application with a route
    let app = Router::new()
    .route("/openapi", get(docs))
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
