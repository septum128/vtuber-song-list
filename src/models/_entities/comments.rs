use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub comment_id: String,
    pub video_id: i64,
    pub author: String,
    pub content: String,
    #[serde(skip)]
    pub response_json: Json,
    pub status: i32,
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
}

impl Related<super::videos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
