use crate::SpotifyGuard;
use rocket::response::Redirect;
use rspotify::prelude::*;

#[get("/")]
pub fn index(spotify: &SpotifyGuard) -> crate::Result<Redirect> {
    let spotify = spotify.lock().unwrap();
    let auth_url = spotify.get_authorize_url(true)?;
    Ok(Redirect::to(auth_url))
}

#[get("/callback?<code>")]
pub fn callback(spotify: &SpotifyGuard, code: &str) -> crate::Result<&'static str> {
    let mut spotify = spotify.lock().unwrap();
    spotify.request_token(code)?;
    Ok("Done!")
}
