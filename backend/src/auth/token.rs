extern crate jsonwebtoken;

use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error};
use lazy_static::lazy_static;
use dotenv_codegen::dotenv;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use super::claims::*;

const VALID_ALGORITHM: Algorithm = Algorithm::HS512;
const COOKIE_PATH: &'static str = "JWT_TOKEN";

lazy_static! {
    static ref SECRET_KEY: String = {
        let path = dotenv!("JWT_KEY_FILE");
        std::fs::read_to_string(path)
            .expect(format!("Failed to read key file {}", path).as_str())
    };
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token(String);

impl Token {
    pub fn from_claims(claims: Claims) -> Result<Token, Error> {
        let header = Header::new(VALID_ALGORITHM);
        let key = EncodingKey::from_secret(SECRET_KEY.as_bytes());
        encode(&header, &claims, &key).map(|token| Token(token))
    }
    pub fn from_string(token: String) -> Token {
        Token(token)
    }
    pub fn claims(self) -> Result<Claims, Error> {
        let key = DecodingKey::from_secret(SECRET_KEY.as_bytes());
        let validation = Validation::new(VALID_ALGORITHM);
        let claims = decode::<Claims>(self.0.as_str(), &key, &validation);
        claims.map(|token| token.claims)
    }
    pub fn is_valid(&self) -> bool {
        let key = DecodingKey::from_secret(SECRET_KEY.as_bytes());
        let validation = Validation::new(VALID_ALGORITHM);
        decode::<Claims>(self.0.as_str(), &key, &validation).is_ok()
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = TokenError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request.cookies().get_private(COOKIE_PATH)
            .map(|token| Token::from_string(token.to_string()))
            .map(|token| {
                if token.is_valid() {
                    Outcome::Success(token)
                } else {
                    Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
                }
            })
            .unwrap_or(Outcome::Failure((Status::Forbidden, TokenError::Missing)))
    }
}

#[test]
fn test_from_user() {
    let token = Token::from_claims(Claims::new(1)).unwrap();
    
    assert!(token.is_valid())
}
