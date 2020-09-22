use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use rocket_contrib::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;
use red_drink::models::{Action, action::*, user::AsUser, traits::*};
use crate::db::Connection;
use crate::auth::Claims;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvokeResponse {
    pub action_id: i32,
    pub executor_id: i32,
    pub invoked_at: NaiveDateTime
}

#[get("/action/<id>/invoke")]
pub fn invoke(claims: Claims, conn: Connection, id: i32) -> Result<Json<InvokeResponse>, Custom<()>> {
    match (claims.as_user(&conn), Action::find(id, &conn)) {
        (Ok(user), Ok(action)) => action.execute(&ExecutableContext { executor: &user, conn: &conn })
            .map(|_| Json(InvokeResponse {
                action_id: id,
                executor_id: user.id,
                invoked_at: Utc::now().naive_utc()
            }))
            .map_err(|err| Custom(match err {
                ExecutableError::AccessDenied => Status::Forbidden,
                _ => Status::InternalServerError
            }, ())),
        _ => Err(Custom(Status::InternalServerError, ()))
    }
}
