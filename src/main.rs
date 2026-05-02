use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt};
use typenx_addon_video_library::{AddonState, app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8790);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let state = Arc::new(AddonState::from_env());
    let router = app(state).layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Typenx addon listening on http://{addr}");
    axum::serve(listener, router).await?;
    Ok(())
}
