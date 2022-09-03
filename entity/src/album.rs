//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "albums")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    #[sea_orm(column_name = "artistName")]
    pub artist_name: String,
    pub cover: Option<String>,
    pub path: String,
    pub year: i32,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: String,
    #[sea_orm(column_name = "updatedAt")]
    pub updated_at: String,
    #[sea_orm(column_name = "artistId")]
    pub artist_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::artist::Entity",
        from = "Column::ArtistId",
        to = "super::artist::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Artist,
    #[sea_orm(has_many = "super::song::Entity")]
    Song,
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl Related<super::song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

/* pub struct SongsForAlbum;

impl Linked for SongsForAlbum {
    type FromEntity = Entity;

    type ToEntity = super::songs::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::albums::Relation::Songs.def(),
            super::songs::Relation::Albums.def().rev(),
        ]
    }
} */
impl ActiveModelBehavior for ActiveModel {}