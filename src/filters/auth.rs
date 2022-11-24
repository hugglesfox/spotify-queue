use crate::auth::Client;
use warp::Filter;
use std::collections::HashMap;
use crate::handlers::auth;

pub fn index(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(with_client(client))
        .map(auth::handle_index)
}

pub fn callback(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("callback")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_client(client))
        .and_then(auth::handle_callback)
}

fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = core::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}
