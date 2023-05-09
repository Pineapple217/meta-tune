use rspotify::ClientCredsSpotify;

#[derive(Clone)]
pub struct AppState {
    pub spotify: ClientCredsSpotify,
}
