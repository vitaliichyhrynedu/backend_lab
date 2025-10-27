use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use uuid::Uuid;

use crate::{AppState, database::Table, models::record::*};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(filter_records).post(create_record))
        .route("/{record_id}", get(get_record).delete(delete_record))
}

async fn get_record(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<RecordBody<Record>> {
    let db = db.read().await;
    let table = match db.get("records") {
        Some(Table::Records(table)) => table,
        _ => panic!("'records' table not found or has wrong type"),
    };
    let record = table.get(&id).unwrap().clone();
    Json(RecordBody { record })
}

async fn create_record(
    State(AppState { db }): State<AppState>,
    Json(RecordBody { record }): Json<RecordBody<RecordCreate>>,
) -> (StatusCode, Json<RecordBody<Record>>) {
    let mut db = db.write().await;
    let table = match db.get_mut("records") {
        Some(Table::Records(table)) => table,
        _ => panic!("'records' table not found or has wrong type"),
    };
    let record = Record {
        id: Uuid::new_v4(),
        user_id: record.user_id,
        category_id: record.category_id,
        created_at: Utc::now(),
        sum: record.sum,
    };
    table.insert(record.id, record.clone());
    (StatusCode::CREATED, Json(RecordBody { record }))
}

async fn delete_record(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut db = db.write().await;
    let table = match db.get_mut("records") {
        Some(Table::Records(table)) => table,
        _ => panic!("'records' table not found or has wrong type"),
    };
    table.remove(&id);
    StatusCode::NO_CONTENT
}

#[derive(serde::Serialize)]
struct ErrorBody {
    error: &'static str,
}

async fn filter_records(
    State(AppState { db }): State<AppState>,
    Query(params): Query<RecordFilterParams>,
) -> Result<Json<RecordsBody<Record>>, (StatusCode, Json<ErrorBody>)> {
    let (user_id, category_id) = match (params.user_id, params.category_id) {
        (None, None) => {
            let error = "at least one filter parameter must be provided";
            Err((StatusCode::UNPROCESSABLE_ENTITY, Json(ErrorBody { error })))
        }
        _ => Ok((params.user_id, params.category_id)),
    }?;

    let db = db.read().await;
    let table = match db.get("records") {
        Some(Table::Records(table)) => table,
        _ => panic!("'records' table not found or has wrong type"),
    };
    let records = table
        .values()
        .filter(|record| user_id.map_or(true, |user_id| record.user_id == user_id))
        .filter(|record| category_id.map_or(true, |category_id| record.category_id == category_id))
        .cloned()
        .collect();

    Ok(Json(RecordsBody { records }))
}
