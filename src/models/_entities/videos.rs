use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "videos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub channel_id: i64,
    #[sea_orm(unique)]
    pub video_id: String,
    pub title: String,
    #[serde(skip)]
    pub response_json: Json,
    pub kind: i32,
    pub status: i32,
    pub published: bool,
    pub published_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channels::Entity",
        from = "Column::ChannelId",
        to = "super::channels::Column::Id"
    )]
    Channel,
    #[sea_orm(has_many = "super::song_items::Entity")]
    SongItems,
    #[sea_orm(has_many = "super::comments::Entity")]
    Comments,
}

impl Related<super::channels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl Related<super::song_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongItems.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
