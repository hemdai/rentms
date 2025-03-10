pub use sea_orm_migration::prelude::*;

mod m20241226_151835_create_user_table;
mod m20250106_211351_create_post_table;
mod m20250109_152030_post_created_at_add;
mod m20250213_205220_update_user_image_active;
mod m20250214_153034_create_property_table;
mod m20250223_111928_create_token;
mod m20250224_153031_create_country_table;
mod m20250224_153032_create_address_table;
mod m20250224_153033_add_address_in_user;
mod m20250224_173314_property_add_address_category_image;
mod m20250303_162302_update_address_foreignkey;
mod m20250308_113425_create_token_type;
mod m20250308_134626_update_token_type_enum;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241226_151835_create_user_table::Migration),
            Box::new(m20250106_211351_create_post_table::Migration),
            Box::new(m20250109_152030_post_created_at_add::Migration),
            Box::new(m20250213_205220_update_user_image_active::Migration),
            Box::new(m20250214_153034_create_property_table::Migration),
            Box::new(m20250223_111928_create_token::Migration),
            Box::new(m20250224_153031_create_country_table::Migration),
            Box::new(m20250224_153032_create_address_table::Migration),
            Box::new(m20250224_153033_add_address_in_user::Migration),
            Box::new(m20250224_173314_property_add_address_category_image::Migration),
            Box::new(m20250303_162302_update_address_foreignkey::Migration),
            Box::new(m20250308_113425_create_token_type::Migration),
            Box::new(m20250308_134626_update_token_type_enum::Migration),
        ]
    }
}
