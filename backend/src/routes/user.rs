use serde::{Serialize, Deserialize};
use json_dotpath::DotPaths;
use rocket_contrib::json::Json;
use crate::types::Session;
use crate::routes::auth::GITHUB_ACCESS_TOKEN_PATH;
use crate::github::create_api_client;
use crate::models::user::User;
use crate::db::Connection;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuthenticatedUserResponse {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub name: String,
    pub email: String
}

#[derive(Serialize)]
pub struct GetTokenResponse {
    pub token: String,
    pub red_drink_id: i32,
    pub username: String,
    pub display_name: String,
    pub id: u64,
    pub avatar_url: String
}

#[get("/token")]
pub fn get_token(session: Session, connection: Connection) -> Json<Option<GetTokenResponse>> {
    Json(session.tap(|data| {
        data.dot_get(GITHUB_ACCESS_TOKEN_PATH).ok().flatten().map(|value: serde_json::Value| {
            value.as_str().map(|token| { token.to_string() })
        }).flatten()
    }).map(|token| {
        create_api_client(&token)
            .get("https://api.github.com/user")
            .send()
            .map(|response| {
                response.json::<GetAuthenticatedUserResponse>().ok().and_then(|authenticated_user_response| {
                    User::find_or_new_by_github_id(authenticated_user_response.id as i32, &connection).map(|user| {
                        GetTokenResponse {
                            red_drink_id: user.id,
                            token: token,
                            username: authenticated_user_response.login,
                            avatar_url: authenticated_user_response.avatar_url,
                            display_name: authenticated_user_response.name,
                            id: authenticated_user_response.id
                        }
                    }).ok()
                })
            }).ok().flatten()
    }).flatten())
}