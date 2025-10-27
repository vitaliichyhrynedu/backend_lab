use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::*;

pub enum Table {
    Users(UserTable),
    Categories(CategoryTable),
    Records(RecordTable),
}

pub type UserTable = HashMap<Uuid, user::User>;

async fn create_users(db: &mut Database) {
    let mut db = db.write().await;
    db.insert("users", Table::Users(UserTable::new()));
}

pub type CategoryTable = HashMap<Uuid, category::Category>;

async fn create_categories(db: &mut Database) {
    let mut db = db.write().await;
    db.insert("categories", Table::Categories(CategoryTable::new()));
}

pub type RecordTable = HashMap<Uuid, record::Record>;

async fn create_records(db: &mut Database) {
    let mut db = db.write().await;
    db.insert("records", Table::Records(RecordTable::new()));
}

pub type Database = Arc<RwLock<HashMap<&'static str, Table>>>;

pub async fn create() -> Database {
    let mut db = Arc::new(RwLock::new(HashMap::new()));
    create_users(&mut db).await;
    create_categories(&mut db).await;
    create_records(&mut db).await;
    db
}
