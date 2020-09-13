use rocket::response::Redirect;
use oauth2::PkceCodeVerifier;
use json_dotpath::DotPaths;
use rocket::http::Cookies;
use crate::auth::Claims;
use crate::github::*;
use crate::types::Session;
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
            create_api_client(&token.secret())
                .get("https://api.github.com/user")
                .send().and_then(|response| {
                    response.json::<GetAuthenticatedUserResponse>()
                        .map(|authenticated_user_response| { (token, authenticated_user_response) })
                }).ok()
        })
        .and_then(|(_, user)| {
            User::find_or_new_by_github_id(user.id as i32, &connection).ok()
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