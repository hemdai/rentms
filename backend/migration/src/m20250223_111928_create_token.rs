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
                    .table(Token::Table)
                    .if_not_exists()
                    .col(pk_auto(Token::Id))
                    .col(string(Token::Key))
                    .col(date_time(Token::CreatedAt).date_time().not_null())
                    .col(integer(Token::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("token_use_id")
                            .from(Token::Table, Token::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Token {
    Table,
    Id,
    Key,
    UserId,
    CreatedAt,
}
