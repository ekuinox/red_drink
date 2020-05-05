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
use oauth2::reqwest::http_client;
use std::env;
use lazy_static::lazy_static;

lazy_static! {
    static ref Client: BasicClient = BasicClient::new(
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
    let (authorize_url, csrf_secret) = Client
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

#[get("/auth?<code>&<state>")]
pub fn auth(code: String, state: String, session: Session) -> Redirect {
    let pkce_verifier = session.tap(|data| {
        PkceCodeVerifier::new((*data).clone())
    });
    let token_result = Client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client).unwrap();

    Redirect::to(format!("/?token={}", token_result.access_token().secret()))
}