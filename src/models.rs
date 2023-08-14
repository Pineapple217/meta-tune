use rspotify::{
    model::{AudioFeatures, Image},
    ClientCredsSpotify,
};
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
    pub release_date: Option<String>,
    pub popularity: u32,
    pub explicit: bool,
    pub url: Option<String>,
    pub preview_url: Option<String>,
    pub genres: Vec<String>,
    pub images: Vec<Image>,
    pub audio_features: AudioFeatures,
}
