use axum::{
    extract::{Path, State},
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info, trace, warn};
use rspotify::{
    model::{FullTrack, TrackId},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

#[derive(Clone)]
struct AppState {
    spotify: ClientCredsSpotify,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let env = Env::default().filter_or("LOG_LEVEL", "warn");
    env_logger::init_from_env(env);

    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();
    let app_state = AppState { spotify: spotify };

    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.js", get(indexmjs_get))
        .route("/index.css", get(indexcss_get))
        .route("/api/track/:id", get(get_track))
        .with_state(app_state.clone());

    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    info!("Listening on {addr}");

    server.await.unwrap();
}

// async fn root_get() -> &'static str {
//     "hi :3"
// }
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();

    Html(markup)
}

async fn indexmjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.js").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

async fn indexcss_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

async fn get_track(Path(id): Path<String>, State(app_state): State<AppState>) -> Json<FullTrack> {
    let spotify = app_state.spotify;
    let track_uri = TrackId::from_id(&id).unwrap();
    let track = spotify.track(track_uri).await.unwrap();
    let artist = spotify
        .artist(track.artists[0].id.as_ref().unwrap().as_ref())
        .await
        .unwrap();
    // format!(
    //     "{} by {}\ngenre: {:?}",
    //     track.name, track.artists[0].name, artist.genres
    // )
    Json(track)
}
