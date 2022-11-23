#[macro_use]
extern crate rocket;

mod routes;

use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use std::sync::{Arc, Mutex};

#[launch]
fn rocket() -> _ {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth {
        redirect_uri: "http://localhost:8000/callback/".to_string(),
        scopes: scopes!("user-modify-playback-state"),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::new(creds, oauth);

    rocket::build().manage(Arc::new(Mutex::new(spotify))).mount(
        "/",
        routes![
            routes::auth::index,
            routes::auth::callback,
            routes::queue::add,
            routes::queue::skip
        ],
    )
}
