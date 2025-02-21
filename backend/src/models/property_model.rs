use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PropertyModel {
    pub id: Uuid,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub price_per_night: i32,
    pub bedroom: i32,
    pub bathroom: i32,
    pub guest: i32,
    pub address_id: Option<i32>,
    pub category: Option<String>,
    pub image: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePropertyModel {
    pub title: String,
    pub description: String,
    pub price_per_night: i32,
    pub bedroom: i32,
    pub bathroom: i32,
    pub guest: i32,
    pub address_id: Option<i32>,
    pub category: Option<String>,
    pub image: Option<String>,
}
