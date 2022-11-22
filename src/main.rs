#[macro_use]
extern crate rocket;

use rocket::{response::Redirect, State};
use rspotify::{
    model::SearchResult, model::SearchType, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth,
};
use std::sync::{Arc, Mutex};

#[get("/callback?<code>")]
fn callback(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>, code: &str) -> &'static str {
    let mut spotify = spotify.lock().unwrap();
    spotify.request_token(code).unwrap();
    "Done!"
}

#[get("/")]
fn index(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>) -> Redirect {
    let spotify = spotify.lock().unwrap();
    let auth_url = spotify.get_authorize_url(true).unwrap();
    Redirect::to(auth_url)
}

#[get("/queue?<q>")]
fn queue(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>, q: &str) -> String {
    let spotify = spotify.lock().unwrap();
    let result = spotify
        .search(q, &SearchType::Track, None, None, Some(1), None)
        .unwrap();
    let track = match result {
        SearchResult::Tracks(page) => {
            Some(page.items[0].clone())
        }
        _ => None,
    }.unwrap();

    spotify.add_item_to_queue(&track.id.unwrap(), None).unwrap();
    format!("Queueing {}", track.name)
}

#[get("/skip")]
fn skip(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>) -> &'static str{
    let spotify = spotify.lock().unwrap();
    spotify.next_track(None).unwrap();
    "Skipped!"
}

#[launch]
fn rocket() -> _ {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth {
        redirect_uri: "http://localhost:8000/callback/".to_string(),
        scopes: scopes!("user-modify-playback-state"),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::new(creds, oauth);

    rocket::build()
        .manage(Arc::new(Mutex::new(spotify)))
        .mount("/", routes![index, callback, queue, skip])
}
