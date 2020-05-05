use rocket::response::Redirect;
use oauth2::{
    AccessToken,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    PkceCodeVerifier,
    RedirectUrl,
    Scope,
    TokenUrl,
    TokenResponse,
    AuthorizationCode
};
use oauth2::basic::BasicClient;
use std::env;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref CLIENT: BasicClient = BasicClient::new(
        ClientId::new(env::var("RED_DRINK_GITHUB_CLIENT_ID").expect("missing RED_DRINK_GITHUB_CLIENT_ID")),
        Some(ClientSecret::new(env::var("RED_DRINK_GITHUB_CLIENT_SECRET").expect("missing RED_DRINK_GITHUB_CLIENT_SECRET"))),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap())
    ).set_redirect_url(RedirectUrl::new(env::var("RED_DRINK_GITHUB_REDIRECT_URL").unwrap().to_string()).unwrap());
}

pub type Session<'a> = rocket_session::Session<'a, String>;

#[get("/get_token")]
pub fn get_token(session: Session) -> Redirect {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_secret) = CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

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
pub fn auth(code: String, state: String, session: Session) -> Redirect {
    let pkce_verifier = session.tap(|data| {
        PkceCodeVerifier::new((*data).clone())
    });
    let token_result = CLIENT
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request(oauth2::reqwest::http_client).unwrap();

    let token = token_result.access_token().secret();
    let client = reqwest::blocking::Client::new();
    let username = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("token {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "red_drink")
        .send().map(|response| {
            if let Ok(authenticated_user_response) = response.json::<GetAuthenticatedUserResponse>() { // when status 200
                authenticated_user_response.login
            } else {
                "".to_string()
            }
        }).unwrap_or("".to_string());

    let url = format!("/?token={}&username={}", token, username);

    Redirect::to(url)
}