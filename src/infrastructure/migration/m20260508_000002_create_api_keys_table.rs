use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut fk = ForeignKey::create();
        fk.name("fk_api_keys_user_id")
            .from(ApiKeys::Table, ApiKeys::UserId)
            .to(Users::Table, Users::Id)
            .on_delete(ForeignKeyAction::Cascade);

        manager
            .create_table(
                Table::create()
                    .table(ApiKeys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiKeys::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiKeys::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(ApiKeys::KeyValue)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(ApiKeys::Label).string().null())
                    .col(
                        ColumnDef::new(ApiKeys::Revoked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(&mut fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiKeys::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ApiKeys {
    Table,
    Id,
    UserId,
    KeyValue,
    Label,
    Revoked,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
