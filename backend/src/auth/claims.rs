use chrono::{Utc, Duration};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use crate::models::user::{User, AsUser};
use crate::db::DBConnection;
use crate::types::DieselError;
use super::{Token, COOKIE_PATH};

/// 有効期間
pub const VALIDITY_DAYS: i64 = 7;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    uid: i32,
    exp: i64
}

impl Claims {
    pub fn new(id: i32) -> Claims {
        Claims {
            uid: id,
            exp: (Utc::now() + Duration::days(VALIDITY_DAYS)).timestamp()
        }
    }
    pub fn to_token(self) -> Result<Token, jsonwebtoken::errors::Error> {
        Token::from_claims(self)
    }
}

impl AsUser<DieselError> for Claims {
    fn as_user(self, conn: &DBConnection) -> Result<User, DieselError> {
        self.uid.as_user(conn)
    }
}

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid
}

impl<'a, 'r> FromRequest<'a, 'r> for Claims {
    type Error = TokenError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request.cookies().get_private(COOKIE_PATH)
            .map(|token| {
                Token::from(token.value().to_string())
            })
            .map(|token| {
                match token.claims() {
                    Ok(claims) => Outcome::Success(claims),
                    Err(_) => Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
                }
            })
            .unwrap_or(Outcome::Failure((Status::Forbidden, TokenError::Missing)))
    }
}
