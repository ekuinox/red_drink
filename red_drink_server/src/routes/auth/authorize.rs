use rocket::response::Redirect;
use oauth2::PkceCodeVerifier;
use json_dotpath::DotPaths;
use rocket::http::Cookies;
use crate::auth::Claims;
use crate::github::*;
use crate::Session;
use crate::db::Connection;
use crate::models::User;
use super::*;

#[derive(Debug, Serialize, Deserialize)]
struct GetAuthenticatedUserResponse {
    login: String,
    id: u64
}

#[get("/auth?<code>")]
pub fn authorize(code: String, session: Session, mut cookies: Cookies, connection: Connection) -> Redirect {
    if let Some(cookie) = session.tap(|data| data.dot_get(PKCE_VERIFIER_PATH).ok().flatten())
        .and_then(|verifier: serde_json::Value| exchange_code_to_access_token(code, PkceCodeVerifier::new(verifier.to_string())))
        .and_then(|token| {
            User::find_or_new_by_github_token(token.secret(), &connection).ok()
        })
        .and_then(|user| {
            Claims::new(user.id).to_token()
                .map(|token| token.to_cookie())
                .ok()
        }) {
            cookies.add_private(cookie);
        }
    Redirect::to("/")
}
