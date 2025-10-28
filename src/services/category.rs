use entity::category;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{error::AppError, models::category::*};

pub async fn get_category(db: &DatabaseConnection, id: Uuid) -> Result<Category, AppError> {
    let category = category::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?
        .into();
    Ok(category)
}

pub async fn create_category(
    db: &DatabaseConnection,
    category: CategoryCreate,
) -> Result<Category, AppError> {
    let category = category::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(category.name),
    };
    let category = category.insert(db).await?.into();
    Ok(category)
}

pub async fn delete_category(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    let res = category::Entity::delete_by_id(id).exec(db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(()),
    }
}

pub async fn get_categories(db: &DatabaseConnection) -> Result<Vec<Category>, AppError> {
    let categories = category::Entity::find()
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(categories)
}
