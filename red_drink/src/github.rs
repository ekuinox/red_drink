use lazy_static::lazy_static;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    PkceCodeVerifier,
    RedirectUrl,
    Scope,
    TokenUrl,
    TokenResponse
};
use url::Url;
use std::env;
use oauth2::basic::BasicClient;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

lazy_static! {
    static ref OAUTH2_CLIENT: BasicClient = BasicClient::new(
        ClientId::new(env::var("RED_DRINK_GITHUB_CLIENT_ID").expect("missing RED_DRINK_GITHUB_CLIENT_ID")),
        Some(ClientSecret::new(env::var("RED_DRINK_GITHUB_CLIENT_SECRET").expect("missing RED_DRINK_GITHUB_CLIENT_SECRET"))),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap())
    ).set_redirect_url(RedirectUrl::new(env::var("RED_DRINK_GITHUB_REDIRECT_URL").unwrap().to_string()).unwrap());

    static ref API_CLIENT: Client = Client::builder()
        .user_agent("red_drink")
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/vnd.github.v3+json".parse().unwrap());
            headers
        })
        .build()
        .unwrap();
}

pub fn get_authorize_url() -> (Url, CsrfToken, PkceCodeChallenge, PkceCodeVerifier) {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_secret) = OAUTH2_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .set_pkce_challenge(pkce_challenge.clone())
        .url();
    (authorize_url, csrf_secret, pkce_challenge, pkce_verifier)
}

// codeからaccess_tokenに変換する
pub fn exchange_code_to_access_token(code: String, pkce_verifier: PkceCodeVerifier) -> Option<oauth2::AccessToken> {
    OAUTH2_CLIENT
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request(oauth2::reqwest::http_client)
        .map(|token| { token.access_token().clone() })
        .ok()
}

// GitHub APIを叩くクライアントを作成する
pub fn create_api_client(access_token: &String) -> Client {
    Client::builder()
        .user_agent("red_drink")
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/vnd.github.v3+json".parse().unwrap());
            headers.insert("Authorization", format!("token {}", access_token).parse().unwrap());
            headers
        })
        .build()
        .unwrap()
}