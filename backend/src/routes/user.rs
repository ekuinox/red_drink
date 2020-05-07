use serde::Serialize;
use json_dotpath::DotPaths;
use rocket_contrib::json::Json;
use crate::types::Session;
use crate::routes::auth::GITHUB_ACCESS_TOKEN_PATH;

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