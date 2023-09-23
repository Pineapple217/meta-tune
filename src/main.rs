mod handlers;
mod models;
use axum::body::Body;
use axum::http::Request;
use tower_http::trace::TraceLayer;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnResponse},
    LatencyUnit,
};
use tracing::{info, metadata::LevelFilter};
use tracing::{Level, Span};
use tracing_subscriber::EnvFilter;

use axum::{routing::get, Router, Server};
use dotenv::dotenv;
use rspotify::{
    // prelude::*,
    ClientCredsSpotify,
    Config,
    Credentials,
};

use models::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::WARN.into())
        .with_env_var("LOG_LEVEL")
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let creds = Credentials::from_env().expect("Spotify credentials");
    let conf = Config {
        token_refreshing: true,
        ..Default::default()
    };
    let analytics_script = std::env::var("ANALYTICS_SCRIPT").unwrap_or("".to_string());
    info!("analytics_script: {analytics_script}");
    let spotify = ClientCredsSpotify::with_config(creds, conf);
    spotify.request_token().await.unwrap();

    let app_state = AppState {
        spotify: spotify,
        analytics_script: analytics_script,
    };

    let router = Router::new()
        .route("/", get(handlers::root_get))
        .route("/index.js", get(handlers::indexmjs_get))
        .route("/index.css", get(handlers::indexcss_get))
        .route("/api/track/:id", get(handlers::get_track))
        .route("/api/spotifylink/:id", get(handlers::get_link))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(true)
                        .level(Level::DEBUG),
                )
                // .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_request(|request: &Request<Body>, _span: &Span| {
                    tracing::info!(
                        "request: {} {} {:?}",
                        request.method(),
                        request.uri().path(),
                        request.headers()
                    )
                })
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(app_state.clone());

    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    info!("Listening on {addr}");

    server
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    info!("signal shutdown");
}
