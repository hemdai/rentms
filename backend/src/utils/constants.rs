use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = get_db_url();
    pub static ref SECRET_KEY: String = get_secret_key();
    pub static ref MAX_FILE_SIZE: u64 = get_max_file_size();
    pub static ref ACCOUNT_AUTHENTICATION_METHOD: String = get_account_authentication_method();
    pub static ref DOMAIN_URL: String = get_domain_url();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or("127.0.0.1".to_owned())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse::<u16>()
        .unwrap()
}

fn get_db_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap_or("DATABASE_URL".to_string())
}

fn get_secret_key() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET_KEY").unwrap_or("SECRET".to_string())
}

fn get_max_file_size() -> u64 {
    dotenv::dotenv().ok();
    env::var("MAX_SIZE_SIZE")
        .unwrap_or("10485760".to_owned())
        .parse::<u64>()
        .expect("Can't parse the port")
}

fn get_account_authentication_method() -> String {
    dotenv::dotenv().ok();
    env::var("ACCOUNT_AUTHENTICATION_METHOD").unwrap_or("email".to_string())
}

fn get_domain_url() -> String {
    dotenv::dotenv().ok();
    env::var("DOMAIN_URL").unwrap_or("/".to_string())
}
