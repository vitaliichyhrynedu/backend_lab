use entity::user;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, SqlErr};
use uuid::Uuid;

use crate::{error::AppError, models::user::*};

pub async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<User, AppError> {
    let user = user::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?
        .into();
    Ok(user)
}

pub async fn create_user(db: &DatabaseConnection, user: UserCreate) -> Result<User, AppError> {
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(user.name),
        default_currency_code: Set(user.default_currency_code),
    };
    let user = user
        .insert(db)
        .await
        .map_err(|e| match e.sql_err() {
            Some(SqlErr::ForeignKeyConstraintViolation(_)) => AppError::unprocessable_entity([(
                "default_currency_code",
                "currency code doesn't exist",
            )]),
            _ => e.into(),
        })?
        .into();
    Ok(user)
}

pub async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    let res = user::Entity::delete_by_id(id).exec(db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(()),
    }
}

pub async fn get_users(db: &DatabaseConnection) -> Result<Vec<User>, AppError> {
    let users = user::Entity::find()
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(users)
}
