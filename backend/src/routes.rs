use rocket::response::Redirect;
use oauth2::{
    AccessToken,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
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

#[get("/get_token")]
pub fn get_token() -> Redirect {
    let (authorize_url, csrf_secret) = Client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();


    Redirect::to(authorize_url.into_string())
}

#[get("/auth?<code>&<state>")]
pub fn auth(code: String, state: String) -> Redirect {
    let token_result = Client
        .exchange_code(AuthorizationCode::new(code))
        .request(http_client).unwrap();

    Redirect::to(format!("/?token={}", token_result.access_token().secret()))
}