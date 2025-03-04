use crate::m20250224_153031_create_country_table::Country;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the existing foreign key constraint if it exists
        manager
            .alter_table(
                Table::alter()
                    .table(Address::Table)
                    .drop_foreign_key(Alias::new("fk-address-country-id"))
                    .to_owned(),
            )
            .await?;

        // Add the new foreign key constraint with ON DELETE CASCADE
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_address_country_id")
                    .from(Address::Table, Address::CountryId)
                    .to(Country::Table, Country::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Optionally, define the down migration to revert the changes
        manager
            .alter_table(
                Table::alter()
                    .table(Address::Table)
                    .drop_foreign_key(Alias::new("fk_address_country_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk-address-country-id")
                    .from(Address::Table, Address::CountryId)
                    .to(Country::Table, Country::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Address {
    Table,
    CountryId,
}
