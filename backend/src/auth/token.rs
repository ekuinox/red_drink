extern crate jsonwebtoken;

use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error};
use lazy_static::lazy_static;
use dotenv_codegen::dotenv;
use super::claims::*;

const VALID_ALGORITHM: Algorithm = Algorithm::HS512;

lazy_static! {
    static ref SECRET_KEY: String = {
        let path = dotenv!("JWT_KEY_FILE");
        std::fs::read_to_string(path)
            .expect(format!("Failed to read key file {}", path).as_str())
    };
}

#[derive(Debug, Eq, PartialEq)]
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
    pub fn is_valid(self) -> bool {
        self.claims().is_ok()
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        self.0
    }
}

#[test]
fn test_from_user() {
    let token = Token::from_claims(Claims::new(1)).unwrap();
    
    assert!(token.is_valid())
}
