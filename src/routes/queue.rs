use rocket::State;
use std::sync::{Arc, Mutex};

use rspotify::{model::SearchResult, model::SearchType, prelude::*, AuthCodeSpotify};

#[get("/queue?<q>")]
pub fn add(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>, q: &str) -> String {
    let spotify = spotify.lock().unwrap();
    let result = spotify
        .search(q, &SearchType::Track, None, None, Some(1), None)
        .unwrap();
    let track = match result {
        SearchResult::Tracks(page) => Some(page.items[0].clone()),
        _ => None,
    }
    .unwrap();

    spotify.add_item_to_queue(&track.id.unwrap(), None).unwrap();
    format!("Queueing {}", track.name)
}

#[get("/queue/skip")]
pub fn skip(spotify: &State<Arc<Mutex<AuthCodeSpotify>>>) -> &'static str {
    let spotify = spotify.lock().unwrap();
    spotify.next_track(None).unwrap();
    "Skipped!"
}
