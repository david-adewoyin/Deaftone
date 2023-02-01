use chrono::Utc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sqlx::{sqlite::SqliteQueryResult, Sqlite, Transaction};

use uuid::Uuid;

use crate::scanner::tag_helper::AudioMetadata;
pub async fn get_song(
    db: &DatabaseConnection,
    id: String,
) -> anyhow::Result<Option<entity::song::Model>, anyhow::Error> {
    match entity::song::Entity::find_by_id(id.to_owned())
        .one(db)
        .await
    {
        Ok(model) => Ok(model),
        Err(err) => anyhow::bail!("Failed to get song: {:}", err),
    }
}

pub async fn like_song(db: &DatabaseConnection, id: String) -> Result<bool, anyhow::Error> {
    /*     Ok(sqlx::query(
        "UPDATE songs
    SET liked = ?
    WHERE id = ?",
    )
    .bind(id)
    .bind(like)
    .await?) */
    let song: Option<entity::song::Model> = entity::song::Entity::find_by_id(id).one(db).await?;
    let mut song: entity::song::ActiveModel = song.unwrap().into();
    match song.liked.unwrap() {
        true => {
            song.liked = Set(false);
        }
        _ => {
            song.liked = Set(true);
        }
    }
    Ok(song.update(db).await?.liked)
}
pub async fn _get_song_by_path(
    db: &DatabaseConnection,
    path: String,
) -> anyhow::Result<Option<entity::song::Model>> {
    let song: Option<entity::song::Model> = entity::song::Entity::find()
        .filter(entity::song::Column::Path.eq(path))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(song)
}

// // Creates a song entry with with the passed album_id and AudioMetadata block
pub async fn create_song(
    tx: &mut Transaction<'_, Sqlite>,
    album_id: &str,
    metadata: &AudioMetadata,
) -> Result<SqliteQueryResult, anyhow::Error> {
    let id: Uuid = Uuid::new_v4();
    let init_time: String = Utc::now().naive_local().to_string();
    Ok(sqlx::query(
        "INSERT OR REPLACE INTO songs (
            id,
            path,
            title,
            artist,
            artist_sort,
            album_name,
            album_artist,
            album_sort,
            discogs_albumid,
            discogs_artistid,
            discogs_labelid,
            lyricist,
            composer,
            composer_sort,
            work,
            mb_workid,
            arranger,
            grouping,
            year,
            lyrics,
            comments,
            bpm,
            comp,
            mb_track_id,
            mb_album_id,
            mb_artist_id,
            mb_albumartist_id,
            mb_releasetrack_id,
            mb_releasegroupid,
            trackdisambig,
            albumtype,
            acoustid_fingerprint,
            acoustid_id,
            asin,
            isrc,
            catalognum,
            script,
            country,
            albumstatus,
            media,
            albumdisambig,
            releasegroupdisambig,
            encoder,
            original_year,
            initial_key,
            encoder_settings,
            track,
            disk,
            length,
            label,
            created_at,
            updated_at,
            album_id,
            liked
         )
    VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
    )
    .bind(id.to_string())
    .bind(&metadata.path)
    .bind(&metadata.name)
    .bind(&metadata.artist)
    .bind(&metadata.artist_sort)
    .bind(&metadata.album_name)
    .bind(&metadata.album_artist)
    .bind(&metadata.album_sort)
    /*     .bind(
            &metadata
                .genre
                .into_iter()
                .map(|i| String::from("{:?}", i))
                .collect::<String>(),
        ) */
    /*     .bind(&metadata.style)
     */
    .bind(&metadata.discogs_albumid)
    .bind(&metadata.discogs_artistid)
    .bind(&metadata.discogs_labelid)
    .bind(&metadata.lyricist)
    .bind(&metadata.composer)
    .bind(&metadata.composer_sort)
    .bind(&metadata.work)
    .bind(&metadata.mb_workid)
    /*     .bind(&metadata.work_disambig)
     */
    .bind(&metadata.arranger)
    .bind(&metadata.grouping)
    .bind(&metadata.year)
    .bind(&metadata.lyrics)
    .bind(&metadata.comments)
    .bind(&metadata.bpm)
    .bind(&metadata.compilation)
    .bind(&metadata.mb_track_id)
    .bind(&metadata.mb_album_id)
    .bind(&metadata.mb_artist_id)
    .bind(&metadata.mb_albumartist_id)
    .bind(&metadata.mb_releasetrack_id)
    .bind(&metadata.mb_releasegroupid)
    .bind(&metadata.trackdisambig)
    .bind(&metadata.album_type)
    /*     .bind(&metadata.albumtypes)
     */
    .bind(&metadata.acoustid_fingerprint)
    .bind(&metadata.acoustid_id)
    .bind(&metadata.asin)
    .bind(&metadata.isrc)
    .bind(&metadata.catalog_num)
    .bind(&metadata.script)
    /*     .bind(&metadata.language)
     */
    .bind(&metadata.country)
    .bind(&metadata.albumstatus)
    .bind(&metadata.media)
    .bind(&metadata.albumdisambig)
    .bind(&metadata.releasegroupdisambig)
    /*     .bind(&metadata.disctitle)
     */
    .bind(&metadata.encodedby)
    .bind(&metadata.original_year)
    .bind(&metadata.initial_key)
    /*     .bind(&metadata.bitrate)
        .bind(&metadata.bitrate_mode) */
    /*     .bind(&metadata.encoder_info)
     */
    .bind(&metadata.encoder_settings)
    /*     .bind(&metadata.format)
    .bind(&metadata.bitdepth)
    .bind(&metadata.channels) */
    .bind(&metadata.track)
    .bind(&metadata.disc)
    /*     .bind(&metadata.codec)
     */
    .bind(&metadata.length)
    .bind(&metadata.label)
    /*     .bind(&metadata.sample_rate)
    .bind(&metadata.bits_per_sample) */
    .bind(&init_time)
    .bind(&init_time)
    .bind(&album_id)
    .bind(false)
    .execute(&mut *tx)
    .await?)
}
