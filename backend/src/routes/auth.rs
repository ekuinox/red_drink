use rocket::response::Redirect;
use serde::{Serialize, Deserialize};
use oauth2::PkceCodeVerifier;
use crate::github::*;
use crate::types::Session;
use json_dotpath::DotPaths;

const PKCE_VERIFIER_PATH: &'static str = "PKCE_VERIFIER";

#[get("/request_token")]
pub fn request_token(session: Session) -> Redirect {
    let (authorize_url, _, _, pkce_verifier) = get_authorize_url();

    let _ = session.tap(|v| {
        v.dot_set(PKCE_VERIFIER_PATH, pkce_verifier.secret().clone())
    });

    Redirect::to(authorize_url.into_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct GetAuthenticatedUserResponse {
    login: String
}

#[get("/auth?<code>&<state>")]
pub fn authorize(code: String, state: String, session: Session) -> Redirect {
    Redirect::to({
        session.tap(|data| {
            data.dot_get(PKCE_VERIFIER_PATH).ok().flatten().map(|verifier: serde_json::Value| { PkceCodeVerifier::new(verifier.as_str().unwrap().to_string()) })
        }).map(|pkce_verifier| { exchange_code_to_access_token(code, pkce_verifier) })
        .flatten()
        .map(|token| {
            create_api_client(&token.secret())
                .get("https://api.github.com/user")
                .send().map(|response| {
                    response.json::<GetAuthenticatedUserResponse>().ok().map(|authenticated_user_response| { (token, authenticated_user_response.login) })
                }).ok().flatten()
        })
        .flatten()
        .map_or(format!("/"), |(token, username)| {
            format!("/?token={}&username={}", token.secret(), username)
        })
    })
}