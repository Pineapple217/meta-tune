use axum::{
    extract::{Path, State},
    http::Response,
    response::{Html, IntoResponse},
    Json,
};
// use log::{debug, error, info, trace, warn};
use rspotify::{
    model::{FullTrack, TrackId},
    prelude::*,
};

use crate::models::AppState;

pub async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();

    Html(markup)
}

pub async fn indexmjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.js").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

pub async fn indexcss_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

pub async fn get_track(
    Path(id): Path<String>,
    State(app_state): State<AppState>,
) -> Json<FullTrack> {
    let spotify = app_state.spotify;
    let track_uri = TrackId::from_id_or_uri(&id).unwrap();
    let track = spotify.track(track_uri).await.unwrap();
    let _artist = spotify
        .artist(track.artists[0].id.as_ref().unwrap().as_ref())
        .await
        .unwrap();
    Json(track)
}
