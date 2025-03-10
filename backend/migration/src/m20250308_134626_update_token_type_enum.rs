use sea_orm::sea_query::extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
use sea_orm::{EnumIter, Iterable};

#[derive(DeriveIden)]
struct TokenTypeEnum;

#[derive(DeriveIden, EnumIter)]
enum TokenType {
    #[sea_orm(iden = "access_token")]
    AccessToken,
    #[sea_orm(iden = "authorized_token")]
    AuthorizedToken,
    #[sea_orm(iden = "app_token")]
    AppToken,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TokenTypeEnum)
                    .values(TokenType::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Token::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Token::NewTokenType)
                            .custom(TokenTypeEnum)
                            .not_null()
                            .default(TokenType::AccessToken.to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
            UPDATE token
            SET new_token_type = 
            CASE
               WHEN type = 'access_token' THEN 'access_token'::token_type_enum
               WHEN type = 'authorized_token' THEN 'authorized_token'::token_type_enum
               WHEN type = 'app_token' THEN 'app_token'::token_type_enum
               ELSE 'access_token'::token_type_enum
            END
            "#,
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Token::Table)
                    .drop_column(Token::Type)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Token::Table)
                    .rename_column(Alias::new("new_token_type"), Token::TokenType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Token::Table)
                    .add_column(
                        ColumnDef::new(Token::Type)
                            .string()
                            .not_null()
                            .default("access_token"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
            UPDATE token 
            SET type = 
            CASE 
                WHEN token_type = 'access_token'::token_type_enum THEN 'access_token'
                WHEN token_type = 'authorized_token'::token_type_enum THEN 'authorized_token'
                WHEN token_type = 'app_token'::token_type_enum THEN 'app_token'
                ELSE 'access_token'
            END
            "#,
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Token::Table)
                    .drop_column(Token::TokenType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Token {
    Table,
    Type,
    TokenType,
    NewTokenType,
}
