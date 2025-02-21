use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CountryModel {
    pub id: i32,
    pub name: String,
    pub iso: String,
    pub currency: String,
    pub phone_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCountryModel {
    pub name: String,
    pub iso: String,
    pub currency: String,
    pub phone_code: String,
}
