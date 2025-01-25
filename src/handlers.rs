use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
// use log::{debug, error, info, trace, warn};
use hyper::header::{HeaderValue, LOCATION, USER_AGENT};
use hyper::http::Uri;
use hyper::Client;
use hyper_tls::HttpsConnector;
use itertools::Itertools;
use regex::Regex;
use rspotify::{model::TrackId, prelude::*};

use crate::models::{AppState, TrackSend};

pub async fn root_get(State(app_state): State<AppState>) -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("static/index.html")
        .await
        .unwrap()
        .replace("{{ analytics }}", &app_state.analytics_script);

    Html(markup)
}

pub async fn indexmjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("static/index.js").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

pub async fn indexcss_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("static/index.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

pub async fn get_link(Path(id): Path<String>) -> Response {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let Ok(initial_url) = Uri::from_str(format!("https://spotify.link/{}", id).as_str()) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let final_url = follow_redirects(&client, initial_url).await;

    let response = client.get(final_url.clone()).await;
    let body_bytes = hyper::body::to_bytes(response.unwrap().into_body())
        .await
        .unwrap();
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();
    let Some(url) = extract_url_from_html(&body) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    Json(url).into_response()
}

pub async fn get_track(Path(id): Path<String>, State(app_state): State<AppState>) -> Response {
    let spotify = app_state.spotify;

    let Ok(track_id) = TrackId::from_id(&id) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(track) = spotify.track(track_id.clone_static(), None).await else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let track_features = spotify.track_features(track_id).await.ok();
    let artists = spotify
        .artists(
            track
                .artists
                .clone()
                .into_iter()
                .map(|artist| artist.id.unwrap()),
        )
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
        release_date: track.album.release_date,
        popularity: track.popularity,
        explicit: track.explicit,
        url: track.external_urls.get("spotify").cloned(),
        preview_url: track.preview_url,
        genres: artists
            .into_iter()
            .map(|art| art.genres)
            .flatten()
            .unique()
            .collect::<Vec<String>>(),
        images: track.album.images,
        audio_features: track_features,
    };
    Json(track_send).into_response()
}

async fn follow_redirects(
    client: &Client<HttpsConnector<hyper::client::HttpConnector>>,
    url: Uri,
) -> Uri {
    let mut current_url = url;

    loop {
        let mut request = hyper::Request::builder()
                .method(hyper::Method::GET)
                .uri(current_url.clone())
                .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36");

        // IDK how user agent works but it works.
        let headers = request.headers_mut().unwrap();
        headers.insert(USER_AGENT, HeaderValue::from_static("User Agent"));

        let response = client
            .request(request.body(hyper::Body::empty()).unwrap())
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_redirection() {
                    if let Some(location) = resp.headers().get(LOCATION) {
                        if let Ok(location) = location.to_str() {
                            current_url = location.parse().unwrap();
                            continue;
                        }
                    }
                } else {
                    return current_url;
                }
            }
            Err(_) => {
                return current_url;
            }
        }
    }
}

fn extract_url_from_html(html: &str) -> Option<String> {
    let re = Regex::new(r"https://open\.spotify\.com/track/[^\s?]+\?si=[^\s&]+").unwrap();

    if let Some(mat) = re.find(html) {
        return Some(mat.as_str().to_string());
    } else {
        return None;
    }
}
