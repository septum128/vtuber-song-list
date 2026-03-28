use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

pub use super::_entities::song_diffs::{self, ActiveModel, Entity, Model};

// status values
pub const STATUS_PENDING: i32 = 0;
pub const STATUS_APPROVED: i32 = 10;
pub const STATUS_REJECTED: i32 = 20;

// kind values
pub const KIND_MANUAL: i32 = 0;
pub const KIND_AUTO: i32 = 10;

pub struct SongDiffsAdminParams {
    pub status: Option<i32>,
    pub page: Option<u64>,
    pub count: Option<u64>,
}

impl SongDiffsAdminParams {
    fn limit(&self) -> u64 {
        self.count.unwrap_or(20)
    }

    fn offset(&self) -> u64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }
}

impl Model {
    /// Returns all song diffs for a song item, ordered by creation time.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_by_song_item(
        db: &DatabaseConnection,
        song_item_id: i64,
    ) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .filter(song_diffs::Column::SongItemId.eq(song_item_id))
            .order_by_asc(song_diffs::Column::CreatedAt)
            .all(db)
            .await
    }

    /// Returns paginated song diffs for admin, with optional status filter.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_all_admin(
        db: &DatabaseConnection,
        params: &SongDiffsAdminParams,
    ) -> Result<Vec<Self>, DbErr> {
        let mut select = Entity::find();
        if let Some(status) = params.status {
            select = select.filter(song_diffs::Column::Status.eq(status));
        }
        select
            .order_by_desc(song_diffs::Column::CreatedAt)
            .limit(params.limit())
            .offset(params.offset())
            .all(db)
            .await
    }

    /// Finds a single song diff by primary key.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    /// Approves a song diff and updates the parent `song_item`'s `latest_diff_id`.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn approve(db: &DatabaseConnection, id: i32) -> Result<Option<Self>, DbErr> {
        use super::_entities::song_items;

        let Some(diff) = Entity::find_by_id(id).one(db).await? else {
            return Ok(None);
        };

        let mut active: ActiveModel = diff.clone().into();
        active.status = ActiveValue::set(STATUS_APPROVED);
        let diff = active.update(db).await?;

        // Update song_item.latest_diff_id to point to this diff.
        #[allow(clippy::cast_possible_truncation)]
        let song_item = song_items::Entity::find_by_id(diff.song_item_id as i32)
            .one(db)
            .await?;
        if let Some(item) = song_item {
            let mut item_active: song_items::ActiveModel = item.into();
            item_active.latest_diff_id = ActiveValue::set(Some(diff.id));
            item_active.update(db).await?;
        }

        Ok(Some(diff))
    }

    /// Rejects a song diff.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn reject(db: &DatabaseConnection, id: i32) -> Result<Option<Self>, DbErr> {
        let Some(diff) = Entity::find_by_id(id).one(db).await? else {
            return Ok(None);
        };

        let mut active: ActiveModel = diff.into();
        active.status = ActiveValue::set(STATUS_REJECTED);
        let diff = active.update(db).await?;
        Ok(Some(diff))
    }
}

pub struct CreateSongDiffParams {
    pub song_item_id: i64,
    pub made_by_id: i32,
    pub time: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub is_admin: bool,
}

impl ActiveModel {
    /// Creates an auto-generated song diff with `kind=auto` and `status=approved`,
    /// then updates the parent `song_item`'s `latest_diff_id`.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn create_auto(
        db: &DatabaseConnection,
        song_item_id: i64,
        comment_id: Option<i64>,
        time: Option<String>,
        title: Option<String>,
        author: Option<String>,
    ) -> Result<Model, DbErr> {
        use super::_entities::song_items;

        let diff = Self {
            song_item_id: ActiveValue::set(song_item_id),
            comment_id: ActiveValue::set(comment_id),
            time: ActiveValue::set(time),
            title: ActiveValue::set(title),
            author: ActiveValue::set(author),
            status: ActiveValue::set(STATUS_APPROVED),
            kind: ActiveValue::set(KIND_AUTO),
            ..Default::default()
        }
        .insert(db)
        .await?;

        #[allow(clippy::cast_possible_truncation)]
        let song_item = song_items::Entity::find_by_id(song_item_id as i32)
            .one(db)
            .await?;
        if let Some(item) = song_item {
            let mut item_active: song_items::ActiveModel = item.into();
            item_active.latest_diff_id = ActiveValue::set(Some(diff.id));
            item_active.update(db).await?;
        }

        Ok(diff)
    }

    /// Creates and inserts a new song diff.
    /// Admin users get `status=approved` immediately and update `song_item.latest_diff_id`.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn create_from_params(
        db: &DatabaseConnection,
        params: CreateSongDiffParams,
    ) -> Result<Model, DbErr> {
        use super::_entities::song_items;

        let status = if params.is_admin {
            STATUS_APPROVED
        } else {
            STATUS_PENDING
        };

        let diff = Self {
            song_item_id: ActiveValue::set(params.song_item_id),
            made_by_id: ActiveValue::set(Some(i64::from(params.made_by_id))),
            time: ActiveValue::set(params.time),
            title: ActiveValue::set(params.title),
            author: ActiveValue::set(params.author),
            status: ActiveValue::set(status),
            kind: ActiveValue::set(KIND_MANUAL),
            ..Default::default()
        }
        .insert(db)
        .await?;

        // When admin approves immediately, update latest_diff_id on the song_item.
        if params.is_admin {
            #[allow(clippy::cast_possible_truncation)]
            let song_item = song_items::Entity::find_by_id(params.song_item_id as i32)
                .one(db)
                .await?;
            if let Some(item) = song_item {
                let mut active: song_items::ActiveModel = item.into();
                active.latest_diff_id = ActiveValue::set(Some(diff.id));
                active.update(db).await?;
            }
        }

        Ok(diff)
    }
}
