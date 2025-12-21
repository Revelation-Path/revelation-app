use axum::{
    Router, extract::Request, http::StatusCode, middleware, response::Response, routing::any
};
use masterror::prelude::*;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;

use auth::AuthLayer;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server_url =
        std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let payments_url =
        std::env::var("PAYMENTS_URL").unwrap_or_else(|_| "http://localhost:3001".into());

    let app = Router::new()
        .route("/api/payments/*path", any(proxy_payments))
        .route("/api/*path", any(proxy_server))
        .layer(middleware::from_fn(AuthLayer::extract_user))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(GatewayState {
            server_url,
            payments_url
        });

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Gateway listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
#[allow(dead_code)]
struct GatewayState {
    server_url:   String,
    payments_url: String
}

async fn proxy_server(
    axum::extract::State(_state): axum::extract::State<GatewayState>,
    request: Request
) -> Result<Response, StatusCode> {
    // TODO: Implement actual proxy using hyper client
    tracing::info!("Proxying to server: {} {}", request.method(), request.uri());
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn proxy_payments(
    axum::extract::State(_state): axum::extract::State<GatewayState>,
    request: Request
) -> Result<Response, StatusCode> {
    // TODO: Implement actual proxy using hyper client
    tracing::info!(
        "Proxying to payments: {} {}",
        request.method(),
        request.uri()
    );
    Err(StatusCode::NOT_IMPLEMENTED)
}
