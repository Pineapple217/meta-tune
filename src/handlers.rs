use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
// use log::{debug, error, info, trace, warn};
use rspotify::{
    model::{FullTrack, TrackId},
    prelude::*,
};

use crate::models::{AppState, TrackSend};

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

pub async fn get_track(Path(id): Path<String>, State(app_state): State<AppState>) -> Response {
    let spotify = app_state.spotify;

    let Ok(track_id) = TrackId::from_id(&id) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(track) = spotify.track(track_id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let artist = spotify
        .artist(track.artists[0].id.as_ref().unwrap().as_ref())
        .await
        .unwrap();
    let track_send = TrackSend {
        name: track.name,
        artists: track
            .artists
            .into_iter()
            .map(|artist| artist.name)
            .collect(),
        duration: track.duration.num_seconds() as i32,
        popularity: track.popularity,
        genres: artist.genres,
    };
    Json(track_send).into_response()
}
