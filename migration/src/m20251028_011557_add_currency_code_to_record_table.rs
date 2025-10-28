use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let currency_code = TableForeignKey::new()
            .name("fk_record_currency")
            .from_tbl(Record::Table)
            .from_col(Record::CurrencyCode)
            .to_tbl(Currency::Table)
            .to_col(Currency::Code)
            .on_delete(ForeignKeyAction::Restrict)
            .on_update(ForeignKeyAction::Restrict)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(Record::Table)
                    .add_column_if_not_exists(string_len(Record::CurrencyCode, 3))
                    .add_foreign_key(&currency_code)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Record::Table)
                    .drop_foreign_key("fk_record_currency")
                    .drop_column(Record::CurrencyCode)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Record {
    Table,
    CurrencyCode,
}

#[derive(DeriveIden)]
enum Currency {
    Table,
    Code,
}
