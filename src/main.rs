use axum::{
    extract::{Path, State},
    routing::get,
    Router, Server,
};
use rspotify::{model::TrackId, prelude::*, ClientCredsSpotify, Credentials};

#[tokio::main]
async fn main() {
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    println!("Getting spotify token...");
    spotify.request_token().await.unwrap();
    println!("Token optained.");

    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/track/:id", get(get_track))
        .with_state(spotify);

    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

async fn root_get() -> &'static str {
    "hi :3"
}

async fn get_track(Path(id): Path<String>, State(spotify): State<ClientCredsSpotify>) -> String {
    let track_uri = TrackId::from_id(&id).unwrap();
    let track = spotify.track(track_uri).await.unwrap();
    let artist = spotify
        .artist(track.artists[0].id.as_ref().unwrap().as_ref())
        .await
        .unwrap();
    format!(
        "{} by {}\ngenre: {:?}",
        track.name, track.artists[0].name, artist.genres
    )
}
