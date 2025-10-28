use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let default_currency_code = TableForeignKey::new()
            .name("fk_user_currency")
            .from_tbl(User::Table)
            .from_col(User::DefaultCurrencyCode)
            .to_tbl(Currency::Table)
            .to_col(Currency::Code)
            .on_delete(ForeignKeyAction::Restrict)
            .on_update(ForeignKeyAction::Restrict)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column_if_not_exists(string_len(User::DefaultCurrencyCode, 3))
                    .add_foreign_key(&default_currency_code)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_foreign_key("fk_user_currency")
                    .drop_column(User::DefaultCurrencyCode)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    DefaultCurrencyCode,
}

#[derive(DeriveIden)]
enum Currency {
    Table,
    Code,
}
