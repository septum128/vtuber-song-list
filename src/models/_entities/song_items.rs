use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "song_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub video_id: i64,
    pub latest_diff_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::videos::Entity",
        from = "Column::VideoId",
        to = "super::videos::Column::Id"
    )]
    Video,
    #[sea_orm(has_many = "super::song_diffs::Entity")]
    SongDiffs,
}

impl Related<super::videos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}

impl Related<super::song_diffs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongDiffs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
