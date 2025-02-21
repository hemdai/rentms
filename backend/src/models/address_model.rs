use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct AddressModel {
    id: i32,
    street: String,
    building_no: i32,
    postal_code: i32,
    country_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAddressModel {
    street: String,
    building_no: i32,
    postal_code: i32,
    country_id: i32,
}
