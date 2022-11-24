mod filters;
mod handlers;
mod spotify;

use reqwest::Client;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;

const SPOTIFY_SCOPE: &str = "user-modify-playback-state";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();


    let client =
        BasicClient::new(
            ClientId::new("client_id".to_string()),
            Some(ClientSecret::new("client_secret".to_string())),
            AuthUrl::new("http://authorize".to_string()).unwrap(),
            Some(TokenUrl::new("http://token".to_string()).unwrap())
        )
        .set_redirect_uri(RedirectUrl::new("http://redirect".to_string()).unwrap());

    let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read".to_string()))
            .add_scope(Scope::new("write".to_string()))
            .url();

    let index = warp::any();
    warp::serve(index).run(([127, 0, 0, 1], 8000)).await
}
