use rand::{distributions::Alphanumeric, Rng};
use reqwest::RequestBuilder;
use std::collections::HashMap;
use warp::http::Uri;
use warp::http::Error;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    state: String,
}

impl Client {
    pub fn new(
        client: reqwest::Client,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        let state = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        Self {
            client,
            client_id,
            client_secret,
            redirect_uri,
            state,
        }
    }

    pub fn authorize_url(&self, scope: &str) -> Result<Uri, Error> {
        let path_and_query = String::from("/authorize?");

        path_and_query.push_str("response_type=code");
        path_and_query.push_str(&format!("&client_id={}", self.client_id));
        path_and_query.push_str(&format!("&scope={}", scope));
        path_and_query.push_str(&format!("&redirect_uri={}", self.redirect_uri));
        path_and_query.push_str(&format!("&state={}", self.state));

        Uri::builder()
            .scheme("https")
            .authority("accounts.spotify.com")
            .path_and_query(path_and_query)
            .build()
    }

    pub fn access_token(self, code: &str, state: &str) -> Result<RequestBuilder, ()> {
        if state != self.state {
            return Err(());
        }

        let mut params = HashMap::new();

        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("redirect_uri", &self.redirect_uri);

        let key = base64::encode(format!("{}:{}", self.client_id, self.client_secret));

        Ok(self
            .client
            .post("https://accounts.spotify.com/api/token")
            .basic_auth::<String, &str>(key, None)
            .form(&params))
    }
}
