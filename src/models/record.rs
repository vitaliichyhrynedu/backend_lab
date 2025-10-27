use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RecordBody<T> {
    pub record: T,
}

#[derive(Serialize)]
pub struct RecordsBody<T> {
    pub records: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct Record {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub sum: Decimal,
}

#[derive(Deserialize)]
pub struct RecordCreate {
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub sum: Decimal,
}

#[derive(Deserialize)]
pub struct RecordFilterParams {
    pub user_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
}
