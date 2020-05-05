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
    let pkce_verifier_opt = session.tap(|data| {
        let result: Option<Option<serde_json::Value>> = data.dot_get(PKCE_VERIFIER_PATH).ok();
        result.flatten().map(|verifier| { PkceCodeVerifier::new(verifier.as_str().unwrap().to_string()) })
    });

    Redirect::to({
        if let Some(pkce_verifier) = pkce_verifier_opt {
            if let Some(token) = exchange_code_to_access_token(code, pkce_verifier) {
                let username = create_api_client(&token.secret())
                    .get("https://api.github.com/user")
                    .send().map(|response| {
                        if let Ok(authenticated_user_response) = response.json::<GetAuthenticatedUserResponse>() { // when status 200
                            authenticated_user_response.login
                        } else {
                            "".to_string()
                        }
                    }).unwrap_or("".to_string());
                format!("/?token={}&username={}", token.secret(), username)
            } else {
                format!("/")
            }
        } else {
            format!("/")
        }
    })
}