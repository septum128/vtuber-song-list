use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "favorites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i64,
    pub song_item_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::song_items::Entity",
        from = "Column::SongItemId",
        to = "super::song_items::Column::Id"
    )]
    SongItem,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::song_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
