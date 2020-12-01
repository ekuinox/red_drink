use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use rocket::response::status::{Custom};
use rocket::http::Status;
use chrono::NaiveDateTime;
use red_drink::models::{Action, action::descriptor::Descriptor, traits::*};
use crate::db::Connection;
use crate::auth::Claims;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicAction {
    pub id: i32,
    pub kind: String,
    pub descriptor: Descriptor,
    pub created_at: NaiveDateTime
}

impl From<Action> for PublicAction {
    fn from(Action { id, kind, descriptor, created_at }: Action) -> Self {
        PublicAction { id, kind, descriptor, created_at }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionResponse {
    pub action: PublicAction
}

/// Actionの作成を行うAPI
/// 作成自体はログインしている場合誰でも行えるように
#[put("/action", format = "json", data = "<descriptor>")]
pub fn put_action(_claims: Claims, descriptor: Json<Descriptor>, conn: Connection) -> Result<Json<PutActionResponse>, Custom<()>> {
    Action::create(descriptor.0, &conn)
        .map(|action| Json(PutActionResponse { action: action.into() }))
        .map_err(|_| Custom(Status::InternalServerError, ()))
}
