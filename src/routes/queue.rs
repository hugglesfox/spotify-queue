use crate::error::Error;
use crate::SpotifyGuard;
use rspotify::AuthCodeSpotify;
use rspotify::{model::SearchResult, model::SearchType, prelude::*};
use rspotify::model::track::FullTrack;

fn search(query: &str, spotify: &AuthCodeSpotify) -> crate::Result<FullTrack> {
    let result = spotify
        .search(query, &SearchType::Track, None, None, Some(1), None)?;
    
    let page = match result {
        SearchResult::Tracks(p) => Ok(p),
        _ => Err(Error::TrackNotFound),
    }?;

    page.items.get(0).ok_or(Error::TrackNotFound).cloned()
}

#[get("/queue?<q>")]
pub fn add(spotify: &SpotifyGuard, q: &str) -> crate::Result<String> {
    let spotify = spotify.lock().unwrap();

    let track = search(q, &spotify)?;
    spotify.add_item_to_queue(&track.id.ok_or(Error::TrackNotFound)?, None)?;

    Ok(format!("Queueing: \"{}\"", track.name))
}

#[get("/skip")]
pub fn skip(spotify: &SpotifyGuard) -> crate::Result<&'static str> {
    let spotify = spotify.lock().unwrap();
    spotify.next_track(None)?;
    Ok("Skipped!")
}
