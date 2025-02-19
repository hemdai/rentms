use crate::m20241226_151835_create_user_table::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Property::Table)
                    .if_not_exists()
                    .col(
                        string(Property::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(integer(Property::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-properties-users-id")
                            .from(Property::Table, Property::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(string(Property::Title))
                    .col(string(Property::Description))
                    .col(integer(Property::PricePerNight).integer())
                    .col(integer(Property::Bedroom).integer())
                    .col(integer(Property::Bathroom).integer())
                    .col(integer(Property::Guest).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Property::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Property {
    Table,
    Id,
    Title,
    Description,
    UserId,
    PricePerNight,
    Bedroom,
    Bathroom,
    Guest,
}
