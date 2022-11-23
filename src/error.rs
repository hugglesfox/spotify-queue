use rocket::{http::Status, response, response::Responder, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Spotify Error {source:?}")]
    Spotify {
        #[from]
        source: rspotify::ClientError,
    },
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'o> {
        match self {
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
