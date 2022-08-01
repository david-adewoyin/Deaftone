use chrono::Utc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

use super::artist_repo;

pub async fn find_by_name(
    db: &DatabaseConnection,
    album_name: String,
) -> anyhow::Result<Option<entity::albums::Model>> {
    entity::albums::Entity::find()
        .filter(entity::albums::Column::Name.eq(album_name))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn create_album(
    db: &DatabaseConnection,
    album_name: String,
    artist_name: String,
    year: Option<i32>,
) -> anyhow::Result<Uuid> {
    let db_album = entity::artists::Entity::find()
        .filter(entity::artists::Column::Name.eq(artist_name.to_owned()))
        .one(db)
        .await?;

    let id = Uuid::new_v4();
    let init_time: String = Utc::now().naive_local().to_string();

    let mut album = entity::albums::ActiveModel {
        id: Set(id.to_string()),
        name: Set(album_name.to_owned()),
        artist_name: Set(artist_name.to_owned()),
        year: Set(year.unwrap_or_default()),
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
        artist_id: NotSet,
    };

    if db_album.is_some() {
        album.set(
            entity::albums::Column::ArtistId,
            Set(db_album.unwrap().id).into_value().unwrap(),
        )
    } else {
        let artist_id = artist_repo::create_artist(db, artist_name)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        album.set(
            entity::albums::Column::ArtistId,
            Set(artist_id.to_string()).into_value().unwrap(),
        )
    }
    album.insert(db).await.map_err(|e| anyhow::anyhow!(e))?;
    return Ok(id);
}