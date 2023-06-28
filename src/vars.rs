use dotenv::dotenv;
use std::env::var;

pub fn service_name() -> String {
    dotenv().ok();
    var("SERVICE_NAME").unwrap_or_else(|_| "rshop".to_string())
}

pub fn log_path() -> String {
    dotenv().ok();
    var("LOG_PATH").unwrap_or_else(|_| "/var/log/rshop/prod.log".to_string())
}

pub fn database_url() -> String {
    dotenv().ok();
    var("DATABASE_URL").expect("DATABASE_URL is not set")
}

pub fn redis_url() -> String {
    dotenv().ok();
    var("REDIS_URL").expect("REDIS_URL is not set")
}

pub fn secret_key() -> String {
    dotenv().ok();
    var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8))
}

pub fn domain() -> String {
    dotenv().ok();
    var("DOMAIN").unwrap_or_else(|_| "localhost".to_string())
}

pub fn cert_kid() -> String {
    dotenv().ok();
    var("KID").unwrap_or_else(|_| "cert_kid".to_string())
}

pub fn port() -> u16 {
    dotenv().ok();
    var("PORT").expect("PORT is not set").parse::<u16>().ok().expect("PORT should be an integer")
}

pub fn domain_url() -> String {
    dotenv().ok();
    var("DOMAIN_URL").unwrap_or_else(|_| "http://0.0.0.0:3000".to_string())
}

#[allow(dead_code)]
pub fn smtp_username() -> String {
    dotenv().ok();
    var("SMTP_USERNAME").expect("SMTP_USERNAME is not set")
}

#[allow(dead_code)]
pub fn smtp_password() -> String {
    dotenv().ok();
    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set")
}

#[allow(dead_code)]
pub fn smtp_host() -> String {
    dotenv().ok();
    var("SMTP_HOST").expect("SMTP_HOST is not set")
}

#[allow(dead_code)]
pub fn smtp_port() -> u16 {
    dotenv().ok();
    var("SMTP_PORT").expect("SMTP_PORT is not set").parse::<u16>().ok().expect("SMTP_PORT should be an integer")
}

#[allow(dead_code)]
pub fn smtp_sender_name() -> String {
    dotenv().ok();
    var("SMTP_SENDER_NAME").expect("SMTP_SENDER_NAME is not set")
}