mod handlers;
mod models;

#[cfg(debug_assertions)]
use dotenv::dotenv;

use axum::{routing::get, Router, Server};
use env_logger::Env;
use log::info;
// use log::{debug, error, info, trace, warn};
use rspotify::{
    // prelude::*,
    ClientCredsSpotify,
    Config,
    Credentials,
};

use models::AppState;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let env = Env::default().filter_or("LOG_LEVEL", "warn");
    env_logger::init_from_env(env);

    let creds = Credentials::from_env().unwrap();
    let conf = Config {
        token_refreshing: true,
        ..Default::default()
    };
    let spotify = ClientCredsSpotify::with_config(creds, conf);
    spotify.request_token().await.unwrap();

    let app_state = AppState { spotify: spotify };

    let router = Router::new()
        .route("/", get(handlers::root_get))
        .route("/index.js", get(handlers::indexmjs_get))
        .route("/index.css", get(handlers::indexcss_get))
        .route("/api/track/:id", get(handlers::get_track))
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
