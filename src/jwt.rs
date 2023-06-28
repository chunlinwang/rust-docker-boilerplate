use serde::{Serialize, Deserialize};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, TokenData};
use chrono::{Local, Duration};
use std::ops::Add;
use std::fs::{OpenOptions, File};
use std::io::Read;
use crate::vars;
use jsonwebtoken::errors::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    aud: String,       // Optional. Audience
    exp: i64,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64,          // Optional. Issued at (as UTC timestamp)
    iss: String,       // Optional. Issuer
    nbf: i64,          // Optional. Not Before (as UTC timestamp)
    sub: String,       // Optional. Subject (whom token refers to)
    company: String,
}

pub fn build_token() -> Result<String, Error> {
    let mut header = Header::new(Algorithm::RS512);
    header.kid = Some(vars::cert_kid());

    let now: chrono::DateTime<Local> = Local::now();

    let my_claims: TokenClaims = TokenClaims {
        aud: "".to_string(),
        exp: now.add(Duration::hours(6)).timestamp(),
        iat: now.timestamp(),
        iss: "".to_string(),
        nbf: now.timestamp(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned()
    };

    let encoding_key: EncodingKey = EncodingKey::from_rsa_pem(include_bytes!("../cert/private.pem")).unwrap();

    encode(&header, &my_claims, &encoding_key)
}

pub fn validate_token(token: String) -> TokenData<TokenClaims> {
    let mut validation: Validation = Validation::new(Algorithm::RS512);
    validation.validate_exp = true;

    let decoding_key: DecodingKey = DecodingKey::from_rsa_pem(include_bytes!("../cert/public.pem")).unwrap();

    let token_data = match decode::<TokenClaims>(token.as_str(), &decoding_key, &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };

    token_data
}

#[cfg(test)]
mod tests_jwt {
    use super::*;

    #[test]
    fn validate_jwt_token() {
        let token = build_token();
        let expected = validate_token(token.unwrap());

        assert_eq!("ACME", expected.claims.company);
    }
}