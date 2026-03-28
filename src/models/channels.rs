use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order,
    QueryFilter, QueryOrder,
};

pub use super::_entities::channels::{self, ActiveModel, Entity, Model};

// kind values
pub const KIND_HIDDEN: i32 = 0;
pub const KIND_PUBLISHED: i32 = 100;

// status values
pub const STATUS_READY: i32 = 0;

pub struct CreateChannelParams {
    pub channel_id: String,
    pub name: Option<String>,
    pub custom_name: String,
    pub twitter_id: Option<String>,
    pub kind: i32,
}

pub struct UpdateChannelParams {
    pub name: Option<Option<String>>,
    pub custom_name: Option<String>,
    pub twitter_id: Option<Option<String>>,
    pub kind: Option<i32>,
}

impl Model {
    /// Returns all published channels in random order.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_all_published(db: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .filter(channels::Column::Kind.eq(KIND_PUBLISHED))
            .order_by(Expr::cust("RANDOM()"), Order::Asc)
            .all(db)
            .await
    }

    /// Returns all channels (including hidden) ordered by id for admin use.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_all_admin(db: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .order_by_asc(channels::Column::Id)
            .all(db)
            .await
    }

    /// Returns all channels (including hidden) ordered by id for RSS fetching.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_all_for_rss(db: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .order_by_asc(channels::Column::Id)
            .all(db)
            .await
    }

    /// Finds a channel by primary key.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    /// Deletes a channel by primary key.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }
}

impl ActiveModel {
    /// Creates and inserts a new channel.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn create_from_params(
        db: &DatabaseConnection,
        params: CreateChannelParams,
    ) -> Result<Model, DbErr> {
        Self {
            channel_id: ActiveValue::set(params.channel_id),
            name: ActiveValue::set(params.name),
            custom_name: ActiveValue::set(params.custom_name),
            twitter_id: ActiveValue::set(params.twitter_id),
            kind: ActiveValue::set(params.kind),
            status: ActiveValue::set(STATUS_READY),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// Updates an existing channel.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn update_from_params(
        db: &DatabaseConnection,
        id: i32,
        params: UpdateChannelParams,
    ) -> Result<Option<Model>, DbErr> {
        let Some(channel) = Entity::find_by_id(id).one(db).await? else {
            return Ok(None);
        };

        let mut active: Self = channel.into();
        if let Some(name) = params.name {
            active.name = ActiveValue::set(name);
        }
        if let Some(custom_name) = params.custom_name {
            active.custom_name = ActiveValue::set(custom_name);
        }
        if let Some(twitter_id) = params.twitter_id {
            active.twitter_id = ActiveValue::set(twitter_id);
        }
        if let Some(kind) = params.kind {
            active.kind = ActiveValue::set(kind);
        }
        let model = active.update(db).await?;
        Ok(Some(model))
    }
}
