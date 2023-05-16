use rspotify::{model::Image, ClientCredsSpotify};
use serde::Serialize;

#[derive(Clone)]
pub struct AppState {
    pub spotify: ClientCredsSpotify,
}
#[derive(Serialize)]
pub struct TrackSend {
    pub name: String,
    pub artists: Vec<String>,
    pub duration: i32,
    pub popularity: u32,
    pub genres: Vec<String>,
    pub images: Vec<Image>,
}
