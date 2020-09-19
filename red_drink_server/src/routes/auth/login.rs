use json_dotpath::DotPaths;
use rocket::response::Redirect;
use crate::github::*;
use crate::Session;
use super::*;

#[get("/login")]
pub fn login(session: Session) -> Redirect {
    let (authorize_url, _, _, pkce_verifier) = get_authorize_url();

    let _ = session.tap(|v| {
        v.dot_set(PKCE_VERIFIER_PATH, pkce_verifier.secret().clone())
    });

    Redirect::to(authorize_url.into_string())
}
