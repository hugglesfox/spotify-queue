use crate::SpotifyGuard;
use rspotify::{model::SearchResult, model::SearchType, prelude::*};

#[get("/queue?<q>")]
pub fn add(spotify: &SpotifyGuard, q: &str) -> crate::Result<String> {
    let spotify = spotify.lock().unwrap();
    let result = spotify
        .search(q, &SearchType::Track, None, None, Some(1), None)?;
    let track = match result {
        SearchResult::Tracks(page) => Some(page.items[0].clone()),
        _ => None,
    }
    .unwrap();

    spotify.add_item_to_queue(&track.id.unwrap(), None)?;
    Ok(format!("Queueing {}", track.name))
}

#[get("/skip")]
pub fn skip(spotify: &SpotifyGuard) -> crate::Result<&'static str> {
    let spotify = spotify.lock().unwrap();
    spotify.next_track(None)?;
    Ok("Skipped!")
}
