use rocket::Request;

#[catch(500)]
pub fn internal_server_error(_: &Request) -> &'static str {
    "Internal error. Please try restarting the application then reauthenticating with Spotify."
}
