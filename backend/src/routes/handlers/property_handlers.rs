use crate::models::property_model::{CreatePropertyModel, PropertyModel};
use crate::services::file_handler_service::process_image;
use crate::utils::{
    api_response::ApiResponse,
    app_state,
    constants::{DOMAIN_URL, MEDIA_DIRECTORY},
    jwt::Claims,
};
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use chrono::NaiveDateTime;
use chrono::Utc;
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
            image: Some(
                format!(
                    "{}/api/v1/static/images/{}",
                    DOMAIN_URL.to_string(),
                    properti.image.unwrap().to_string()
                )
                .to_string(),
            ),
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
    property_model: MultipartForm<CreatePropertyModel>,
) -> Result<ApiResponse, ApiResponse> {
    let time_created: Option<NaiveDateTime> = Some(Utc::now().naive_local());
    let mut propety_entity = entity::property::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(property_model.title.clone()),
        description: Set(property_model.description.clone()),
        price_per_night: Set(property_model.price_per_night.clone()),
        bathroom: Set(property_model.bathroom.clone()),
        bedroom: Set(property_model.bathroom.clone()),
        address_id: Set(Some(property_model.address_id.clone())),
        category: Set(Some(property_model.category.clone())),
        guest: Set(property_model.guest.clone()),
        user_id: Set(claim.id),
        created_at: Set(time_created),
        ..Default::default()
    };
    let image_name =
        process_image(property_model).map_err(|err| ApiResponse::new(500, err.to_string()))?;

    propety_entity.image = Set(Some(image_name));
    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let created_entity = propety_entity
        .insert(&txn)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))
        .unwrap();
    txn.commit()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    let property_model = PropertyModel {
        id: created_entity.id,
        title: created_entity.title,
        description: created_entity.description,
        price_per_night: created_entity.price_per_night,
        guest: created_entity.guest,
        address_id: created_entity.address_id,
        category: created_entity.category,
        image: Some(
            format!(
                "{}/api/v1/static/images/{}",
                DOMAIN_URL.to_string(),
                created_entity.image.unwrap().to_string()
            )
            .to_string(),
        ),
        created_at: created_entity.created_at,
        user_id: created_entity.user_id,
        bedroom: created_entity.bedroom,
        bathroom: created_entity.bathroom,
    };

    let serialize_record = serde_json::to_string(&property_model)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, serialize_record))
}
