use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "song_diffs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub song_item_id: i64,
    pub made_by_id: Option<i64>,
    pub comment_id: Option<i64>,
    pub time: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub status: i32,
    pub kind: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::song_items::Entity",
        from = "Column::SongItemId",
        to = "super::song_items::Column::Id"
    )]
    SongItem,
}

impl Related<super::song_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
