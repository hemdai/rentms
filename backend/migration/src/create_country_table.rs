use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(pk_auto(Country::Id))
                    .col(string(Country::Name).not_null())
                    .col(string(Country::ISO))
                    .col(string(Country::Currency))
                    .col(string(Country::PhoneCode))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Country {
    Table,
    Id,
    Name,
    ISO,
    Currency,
    PhoneCode,
}
