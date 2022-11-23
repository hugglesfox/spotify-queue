use rocket::{response::Redirect, State};
use rspotify::{prelude::*, AuthCodeSpotify};
use std::sync::{Arc, Mutex};

#[get("/")]
pub fn index(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>) -> Redirect {
    let spotify = spotify.lock().unwrap();
    let auth_url = spotify.get_authorize_url(true).unwrap();
    Redirect::to(auth_url)
}

#[get("/callback?<code>")]
pub fn callback(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>, code: &str) -> &'static str {
    let mut spotify = spotify.lock().unwrap();
    spotify.request_token(code).unwrap();
    "Done!"
}
