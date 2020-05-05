use rocket::response::Redirect;
use serde::{Serialize, Deserialize};
use oauth2::PkceCodeVerifier;
use crate::github::*;
use crate::types::Session;

#[get("/request_token")]
pub fn request_token(session: Session) -> Redirect {
    let (authorize_url, _, _, pkce_verifier) = get_authorize_url();

    session.tap(|v| {
        *v = pkce_verifier.secret().clone();
    });

    Redirect::to(authorize_url.into_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct GetAuthenticatedUserResponse {
    login: String
}

#[get("/auth?<code>&<state>")]
pub fn authorize(code: String, state: String, session: Session) -> Redirect {
    let access_token = exchange_code_to_access_token(code, session.tap(|data| {
        PkceCodeVerifier::new((*data).clone())
    }));

    Redirect::to({
        if let Some(token) = access_token {
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
    })
}