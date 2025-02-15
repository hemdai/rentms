pub use sea_orm_migration::prelude::*;

mod m20241226_151835_create_user_table;
mod m20250106_211351_create_post_table;
mod m20250109_152030_post_created_at_add;
mod m20250213_205220_update_user_image_active;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241226_151835_create_user_table::Migration),
            Box::new(m20250106_211351_create_post_table::Migration),
            Box::new(m20250109_152030_post_created_at_add::Migration),
            Box::new(m20250213_205220_update_user_image_active::Migration),
        ]
    }
}
