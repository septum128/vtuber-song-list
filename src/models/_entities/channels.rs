use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "channels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub channel_id: String,
    pub name: Option<String>,
    pub custom_name: String,
    pub twitter_id: Option<String>,
    #[serde(skip)]
    pub response_json: Option<Json>,
    pub icon_url: Option<String>,
    pub kind: i32,
    pub status: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::videos::Entity")]
    Videos,
}

impl Related<super::videos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Videos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
