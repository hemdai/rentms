use crate::models::property_model::{CreatePropertyModel, PropertyModel};
use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};
use actix_web::{get, post, web};
use chrono::NaiveDateTime;
use chrono::Utc;
use entity::address::ActiveModel;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

#[get("/all-property")]
pub async fn get_all_property(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let all_property: Vec<PropertyModel> = entity::property::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|properti| PropertyModel {
            id: properti.id,
            title: properti.title,
            description: properti.description,
            price_per_night: properti.price_per_night,
            guest: properti.guest,
            address_id: properti.address_id,
            category: properti.category,
            image: properti.image,
            created_at: properti.created_at,
            user_id: properti.user_id,
            bedroom: properti.bedroom,
            bathroom: properti.bathroom,
        })
        .collect();
    let mut string_result = serde_json::to_string(&all_property)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    if string_result.is_empty() || string_result == "[]" {
        string_result = "empty value found ".to_string()
    }

    Ok(ApiResponse::new(200, string_result.to_owned()))
}

#[post("/create-property")]
pub async fn create_property(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    property_model: web::Json<CreatePropertyModel>,
) -> Result<ApiResponse, ApiResponse> {
    let time_created: Option<NaiveDateTime> = Some(Utc::now().naive_local());
    let address: Option<i32> = property_model.address_id;
    let propety_entity = entity::property::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(property_model.title.clone()),
        description: Set(property_model.description.clone()),
        price_per_night: Set(property_model.price_per_night.clone()),
        bathroom: Set(property_model.bathroom.clone()),
        bedroom: Set(property_model.bathroom.clone()),
        address_id: Set(address.clone()),
        category: Set(property_model.category.clone()),
        guest: Set(property_model.guest.clone()),
        user_id: Set(claim.id),
        image: Set(property_model.image.clone()),
        created_at: Set(time_created),
    };

    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let created_entity = propety_entity
        .insert(&txn)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let serialize_record = serde_json::to_string(&created_entity)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, serialize_record))
}
