use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Token::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Token::Type)
                            .string()
                            .default("access_token".to_string())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Token::Table)
                    .drop_column(Token::Type)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Token {
    Table,
    Type,
}
