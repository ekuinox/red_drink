use rocket::response::Redirect;
use serde::{Serialize, Deserialize};
use oauth2::PkceCodeVerifier;
use crate::github::*;
use crate::types::Session;
use json_dotpath::DotPaths;
use rocket_contrib::json::Json;

const PKCE_VERIFIER_PATH: &'static str = "PKCE_VERIFIER";
pub const GITHUB_ACCESS_TOKEN_PATH: &'static str = "GITHUB_ACCESS_TOKEN";

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
    login: String,
    id: u64
}

#[get("/auth?<code>&<state>")]
pub fn authorize(code: String, state: String, session: Session) -> Redirect {
    if let Some((token, authenticate_user_response)) = session.tap(|data| {
        data.dot_get(PKCE_VERIFIER_PATH).ok().flatten().map(|verifier: serde_json::Value| { PkceCodeVerifier::new(verifier.as_str().unwrap().to_string()) })
    }).map(|pkce_verifier| { exchange_code_to_access_token(code, pkce_verifier) })
    .flatten()
    .map(|token| {
        create_api_client(&token.secret())
            .get("https://api.github.com/user")
            .send().map(|response| {
                response.json::<GetAuthenticatedUserResponse>().ok().map(|authenticated_user_response| { (token, authenticated_user_response) })
            }).ok().flatten()
    })
    .flatten() {
        let _ = session.tap(|data| {
            data.dot_set(GITHUB_ACCESS_TOKEN_PATH, token)
        });
    }
    Redirect::to(format!("/"))
}

#[derive(Serialize)]
pub struct GetTokenResponse {
    pub token: Option<String>
}

#[get("/token")]
pub fn get_token(session: Session) -> Json<GetTokenResponse> {
    Json(GetTokenResponse {
        token: session.tap(|data| {
            data.dot_get(GITHUB_ACCESS_TOKEN_PATH).ok().flatten().map(|value: serde_json::Value| {
                value.as_str().map(|token| { token.to_string() })
            }).flatten()
        })
    })
}