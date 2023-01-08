use anyhow::Ok;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use sea_orm::{PaginatorTrait, QuerySelect};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn get_album_by_id(
    db: &DatabaseConnection,
    album_id: String,
) -> anyhow::Result<Vec<(entity::album::Model, Vec<entity::song::Model>)>> {
    Ok(entity::album::Entity::find_by_id(album_id)
        .find_with_related(entity::song::Entity)
        .all(db)
        .await?)
}
pub async fn _find_by_name(
    db: &DatabaseConnection,
    album_name: String,
) -> anyhow::Result<Option<entity::album::Model>> {
    entity::album::Entity::find()
        .filter(entity::album::Column::Name.eq(album_name))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn _update_cover_for_path(
    db: &DatabaseConnection,
    cover_path: String,
    album_path: String,
) -> anyhow::Result<()> {
    let db_album = entity::album::Entity::find()
        .filter(entity::album::Column::Path.eq(album_path))
        .one(db)
        .await?;
    if db_album.is_some() {
        let mut album: entity::album::ActiveModel = db_album.unwrap().into();
        album.cover = Set(Some(cover_path));
        album.update(db).await?;
    }
    Ok(())
}

pub async fn get_all_albums(
    db: &DatabaseConnection,
    size: Option<u64>,
    sort: Option<String>,
) -> anyhow::Result<Vec<entity::album::Model>> {
    let order = match sort.unwrap_or_default().as_str() {
        "name" => entity::album::Column::Name,
        "artist_name" => entity::album::Column::ArtistName,
        "year" => entity::album::Column::Year,
        "latest" => entity::album::Column::CreatedAt,
        _ => entity::album::Column::Name,
    };

    match order {
        entity::album::Column::CreatedAt => Ok(entity::album::Entity::find()
            .order_by_desc(order)
            .limit(size.unwrap_or(u64::MAX))
            .all(db)
            .await
            .expect("Failed to get albums")),
        _ => Ok(entity::album::Entity::find()
            .order_by_asc(order)
            .limit(size.unwrap_or(u64::MAX))
            .all(db)
            .await
            .expect("Failed to get albums")),
    }
}

pub async fn get_latest_albums(
    db: &DatabaseConnection,
    size: Option<u64>,
) -> anyhow::Result<Vec<entity::album::Model>> {
    Ok(entity::album::Entity::find()
        .order_by_desc(entity::album::Column::CreatedAt)
        .all(db)
        .await?)
}
pub async fn get_albums_paginate(
    db: &DatabaseConnection,
    page: u64,
    size: u64,
) -> anyhow::Result<Vec<entity::album::Model>> {
    let db_albums = entity::album::Entity::find()
        .order_by_asc(entity::album::Column::Name)
        .paginate(db, size);
    Ok(db_albums.fetch_page(page).await?)
}
pub async fn create_album(
    tx: &mut Transaction<'_, Sqlite>,
    cover: Option<String>,
    artist_id: &String,
    album_name: &String,
    artist_name: &String,
    path: &String,
    year: &i32,
) -> Result<String, anyhow::Error> {
    let id: String = Uuid::new_v4().to_string();
    let init_time: String = Utc::now().naive_local().to_string();
    sqlx::query(
        "INSERT OR REPLACE INTO albums (
            id, 
            name,
            artistName,
            cover,
            path,
            year,
            createdAt,
            updatedAt,
            artistId
         )
    VALUES (?,?,?,?,?,?,?,?,?)",
    )
    .bind(&id)
    .bind(album_name)
    .bind(artist_name)
    .bind(cover)
    .bind(path)
    .bind(year)
    .bind(&init_time)
    .bind(&init_time)
    .bind(artist_id)
    .execute(&mut *tx)
    .await?;
    Ok(id)
}
