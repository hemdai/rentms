use crate::models::address_model::AddressResponse;
use crate::models::address_model::{AddressModel, CreateAddressModel};
use crate::utils::{api_response::ApiResponse, app_state};
use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};

#[get("all-address")]
pub async fn get_address(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let address_records: Vec<AddressModel> = entity::address::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|address| AddressModel {
            id: address.id,
            street: address.street,
            building_no: address.building_no,
            postal_code: address.postal_code,
            country_id: address.country_id,
        })
        .collect();

    let string_records = serde_json::to_string(&address_records)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, string_records.to_owned()))
}

#[post("/create-address")]
pub async fn create_address(
    app_state: web::Data<app_state::AppState>,
    address_model: web::Json<CreateAddressModel>,
) -> Result<ApiResponse, ApiResponse> {
    let country_model = entity::country::Entity::find_by_id(address_model.country_id.clone())
        .one(&app_state.db)
        .await
        .map_err(|error| ApiResponse::new(500, error.to_string()))?
        .unwrap();

    let address_entity = entity::address::ActiveModel {
        building_no: Set(address_model.building_no.clone()),
        country_id: Set(country_model.id),
        street: Set(address_model.street.clone()),
        postal_code: Set(address_model.postal_code.clone()),
        ..Default::default()
    };
    println!("{:?}", &address_entity);

    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let address_records = address_entity
        .save(&txn)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    txn.commit()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let address_model = AddressModel {
        id: address_records.id.unwrap(),
        building_no: address_records.building_no.unwrap(),
        country_id: address_records.country_id.unwrap(),
        postal_code: address_records.postal_code.unwrap(),
        street: address_records.street.unwrap(),
    };
    let string_record = serde_json::to_string(&address_model)
        .map_err(|error| ApiResponse::new(500, error.to_string()))?;

    Ok(ApiResponse::new(200, string_record))
}
