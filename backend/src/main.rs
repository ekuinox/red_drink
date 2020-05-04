extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;

use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
    PkceCodeChallenge
};
use oauth2::basic::BasicClient;
use dotenv::dotenv;
use oauth2::reqwest::http_client;
use std::env;
use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .launch();

    /*
    dotenv().ok();

    let client_id = ClientId::new(env::var("RED_DRINK_GITHUB_CLIENT_ID").expect("missing RED_DRINK_GITHUB_CLIENT_ID"));
    let client_secret = ClientSecret::new(env::var("RED_DRINK_GITHUB_CLIENT_SECRET").expect("missing RED_DRINK_GITHUB_CLIENT_SECRET"));
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap();
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap();

    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url)
    ).set_redirect_url(RedirectUrl::new("http://localhost:8080".to_string()).unwrap());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    
    let (authorize_url, csrf_secret) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    
    println!("{}", authorize_url);

    let token_result = client
        .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client).unwrap();

    println!("{}", token_result.access_token().secret());
    */
}