pub use sea_orm_migration::prelude::*;

mod m20251027_233718_create_users_table;
mod m20251027_234532_create_categories_table;
mod m20251027_234708_create_records_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251027_233718_create_users_table::Migration),
            Box::new(m20251027_234532_create_categories_table::Migration),
            Box::new(m20251027_234708_create_records_table::Migration),
        ]
    }
}
