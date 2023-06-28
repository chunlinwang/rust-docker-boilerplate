use serde::{Serialize, Deserialize};
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

    let now = Local::now();

    let my_claims: TokenClaims = TokenClaims {
        aud: "".to_string(),
        exp: now.add(Duration::hours(6)).timestamp(),
        iat: now.timestamp(),
        iss: "".to_string(),
        nbf: now.timestamp(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned()
    };

    let encodingKey: EncodingKey = EncodingKey::from_rsa_pem(include_bytes!("../cert/private.pem"))?;

    encode(&header, &my_claims, &encodingKey)
}

pub fn validate_token(token: String) -> Result<TokenData<TokenClaims>, Error> {
    let mut validation: Validation = jsonwebtoken::Validation::new(Algorithm::RS512);
    validation.validate_exp = true;

    let decodingKey: DecodingKey = DecodingKey::from_rsa_pem(include_bytes!("../cert/public.pem"))?;

    decode::<TokenClaims>(token.as_str(), &decodingKey, &validation)
}