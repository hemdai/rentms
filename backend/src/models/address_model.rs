use super::country_model::CountryModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct AddressModel {
    pub id: i32,
    pub street: String,
    pub building_no: i32,
    pub postal_code: i32,
    pub country_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAddressModel {
    pub street: String,
    pub building_no: i32,
    pub postal_code: i32,
    pub country_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AddressResponse {
    address: AddressModel,
    country: CountryModel,
}
