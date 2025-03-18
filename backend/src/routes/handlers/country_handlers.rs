use crate::models;
use crate::models::country_model::{CountryModel, CreateCountryModel};
use crate::utils::{api_response::ApiResponse, app_state};
use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

#[post("/create-country")]
pub async fn create_country(
    app_state: web::Data<app_state::AppState>,
    country_model: web::Json<CreateCountryModel>,
) -> Result<ApiResponse, ApiResponse> {
    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let country_entity = entity::country::ActiveModel {
        name: Set(country_model.name.clone()),
        iso: Set(country_model.iso.clone()),
        currency: Set(country_model.currency.clone()),
        phone_code: Set(country_model.phone_code.clone()),
        ..Default::default()
    };

    let country_record = country_entity
        .save(&txn)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    txn.commit()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let country_model = models::country_model::CountryModel {
        id: country_record.id.unwrap(),
        name: country_record.name.unwrap(),
        iso: country_record.iso.unwrap(),
        currency: country_record.currency.unwrap(),
        phone_code: country_record.phone_code.unwrap(),
    };

    let string_record = serde_json::to_string(&country_model)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, string_record.to_owned()))
}

#[get("/{country_name}")]
pub async fn get_all_country(
    app_state: web::Data<app_state::AppState>,
    country_name: web::Path<String>,
) -> Result<ApiResponse, ApiResponse> {
    if country_name.is_empty() {
        let all_country: Vec<CountryModel> = entity::country::Entity::find()
            .all(&app_state.db)
            .await
            .map_err(|err| ApiResponse::new(500, err.to_string()))?
            .into_iter()
            .map(|country| CountryModel {
                id: country.id,
                name: country.name,
                iso: country.iso,
                currency: country.currency,
                phone_code: country.phone_code,
            })
            .collect();
        let string_result = serde_json::to_string(&all_country)
            .map_err(|err| ApiResponse::new(500, err.to_string()))?;
        Ok(ApiResponse::new(200, string_result.to_owned()))
    } else {
        let country_record = entity::country::Entity::find()
            .filter(entity::country::Column::Name.eq(country_name.to_string()))
            .one(&app_state.db)
            .await
            .map_err(|err| ApiResponse::new(500, err.to_string()))
            .unwrap();
        match country_record {
            None => {
                return Ok(ApiResponse::new(404, "Country not found".to_string()));
            }
            Some(country_record) => {
                let country_model = models::country_model::CountryModel {
                    id: country_record.id,
                    name: country_record.name,
                    iso: country_record.iso,
                    currency: country_record.currency,
                    phone_code: country_record.phone_code,
                };
                let string_result = serde_json::to_string(&country_model)
                    .map_err(|err| ApiResponse::new(500, err.to_string()))?;
                Ok(ApiResponse::new(200, string_result.to_owned()))
            }
        }
    }
}
