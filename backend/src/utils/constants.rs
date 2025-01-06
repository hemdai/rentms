use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = get_db_url();
    pub static ref SECRET_KEY: String = get_secret_key();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}

fn get_db_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn get_secret_key() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET_KEY").unwrap()
}
