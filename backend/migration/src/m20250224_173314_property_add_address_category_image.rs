use crate::m20250224_153032_create_address_table::Address;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Property::Table)
                    .add_column_if_not_exists(ColumnDef::new(Property::AddressId).integer())
                    .add_column_if_not_exists(ColumnDef::new(Property::Category).string())
                    .add_column_if_not_exists(ColumnDef::new(Property::Image).string())
                    .add_column_if_not_exists(ColumnDef::new(Property::CreatedAt).date_time())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_properties_address_id")
                    .from(Property::Table, Property::AddressId)
                    .to(Address::Table, Address::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Property::Table)
                    .drop_column(Property::AddressId)
                    .drop_column(Property::Category)
                    .drop_column(Property::Image)
                    .drop_column(Property::CreatedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Property {
    Table,
    AddressId,
    Category,
    Image,
    CreatedAt,
}
