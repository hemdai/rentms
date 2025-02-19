use crate::create_country_table::Country;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(pk_auto(Address::Id))
                    .col(string(Address::Street))
                    .col(integer(Address::BuildingNo).integer())
                    .col(integer(Address::PostalCode).integer())
                    .col(integer(Address::CountryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-address-country-id")
                            .from(Address::Table, Address::CountryId)
                            .to(Country::Table, Country::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Address::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Address {
    Table,
    Id,
    Street,
    BuildingNo,
    PostalCode,
    CountryId,
}
