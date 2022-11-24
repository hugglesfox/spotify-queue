#[macro_use]
extern crate rocket;

mod error;
mod routes;

use rocket::State;
use rspotify::clients::oauth::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth, Config};
use std::sync::{Arc, Mutex};

type SpotifyGuard = State<Arc<Mutex<AuthCodeSpotify>>>;
type Result<T> = std::result::Result<T, error::Error>;

#[launch]
fn rocket() -> _ {
    let config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth {
        redirect_uri: "http://localhost:8000/callback/".to_string(),
        scopes: scopes!("user-modify-playback-state"),
        ..Default::default()
    };

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    rocket::build()
        .manage(Arc::new(Mutex::new(spotify)))
        .register("/", catchers![routes::error::internal_server_error])
        .mount(
            "/",
            routes![
                routes::auth::index,
                routes::auth::callback,
                routes::queue::add,
                routes::queue::skip,
            ],
        )
}
