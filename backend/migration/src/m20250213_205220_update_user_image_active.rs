use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(User::Avatar)
                            .string()
                            .default("".to_string())
                            .not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(User::IsSuperuser)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(User::IsStaff)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Avatar,
    IsActive,
    IsSuperuser,
    IsStaff,
}
