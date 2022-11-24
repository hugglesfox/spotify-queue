use rocket::{http::Status, response, response::status, response::Responder, Request};
use rspotify::ClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Spotify Error {source:?}")]
    Spotify {
        #[from]
        source: ClientError,
    },

    #[error("No tracks were found")]
    TrackNotFound,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'o> {
        match self {
            Error::TrackNotFound => status::Custom(Status::NotFound, "Track not found").respond_to(req),
            Error::Spotify { source: ClientError::Http(_) } => status::Custom(Status::BadRequest, "Spotify error. Is the player open and playing?").respond_to(req),
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
