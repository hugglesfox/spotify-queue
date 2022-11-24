use oath2::CsrfToken;
use oath2::Scope;
use oauth2::basic::BasicClient;
use warp::http::Error;
use std::collections::HashMap;
use rspotify_model::auth::Token;
use warp::reply;
use warp::http::StatusCode;

pub fn handle_index(oauth: BasicClient) -> Result<impl warp::Reply, Error> {
    let (auth_url, csrf_token) = oauth
            .authorize_url(CsrfToken::new_random)
            // .add_scope(Scope::new("read".to_string()))
            .url();
    Ok(warp::redirect(auth_url))
}

pub async fn handle_callback(query: HashMap<String, String>, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    let code = query.get("code").unwrap();
    let state = query.get("state").unwrap();
    let resp = client.access_token(code, state).unwrap().send().await.unwrap().json::<Token>().await.unwrap();

    Ok(reply::with_status("Done!", StatusCode::OK))
}
