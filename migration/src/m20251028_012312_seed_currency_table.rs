use entity::currency;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveValue::Set, EntityTrait},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let usd = currency::ActiveModel {
            code: Set("USD".to_string()),
            name: Set("US Dollar".to_string()),
            symbol: Set("$".to_string()),
        };
        currency::Entity::insert(usd).exec(db).await?;

        let eur = currency::ActiveModel {
            code: Set("EUR".to_string()),
            name: Set("Euro".to_string()),
            symbol: Set("€".to_string()),
        };
        currency::Entity::insert(eur).exec(db).await?;

        let uah = currency::ActiveModel {
            code: Set("UAH".to_string()),
            name: Set("Hryvnia".to_string()),
            symbol: Set("₴".to_string()),
        };
        currency::Entity::insert(uah).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        currency::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
