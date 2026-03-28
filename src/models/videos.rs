use chrono::NaiveDate;
use sea_orm::sea_query::{Expr, Query};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

use super::_entities::song_items;
pub use super::_entities::videos::{self, ActiveModel, Entity, Model};

pub const DEFAULT_PAGE_SIZE: u64 = 20;

pub const STATUS_READY: i32 = 0;
pub const STATUS_FETCHED: i32 = 10;
pub const STATUS_COMMENTS_DISABLED: i32 = 11;
pub const STATUS_SONG_ITEMS_CREATED: i32 = 20;
pub const STATUS_FETCHED_HISTORY: i32 = 25;
pub const STATUS_SPOTIFY_FETCHED: i32 = 30;
pub const STATUS_SPOTIFY_COMPLETED: i32 = 35;
pub const STATUS_COMPLETED: i32 = 40;

pub struct VideosParams {
    pub channel_id: Option<i64>,
    pub query: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub only_song_lives: Option<i64>,
    pub page: Option<u64>,
    pub count: Option<u64>,
}

pub struct VideosAdminParams {
    pub channel_id: Option<i64>,
    pub only_song_lives: Option<bool>,
    pub page: Option<u64>,
    pub count: Option<u64>,
}

pub struct CreateVideoParams {
    pub video_id: String,
    pub channel_id: i64,
    pub title: String,
    pub published_at: sea_orm::prelude::DateTimeWithTimeZone,
    pub response_json: serde_json::Value,
}

pub struct UpdateVideoParams {
    pub title: Option<String>,
    pub published: Option<bool>,
    pub kind: Option<i32>,
    pub status: Option<i32>,
}

impl VideosParams {
    fn limit(&self) -> u64 {
        self.count.unwrap_or(DEFAULT_PAGE_SIZE)
    }

    fn offset(&self) -> u64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }
}

impl VideosAdminParams {
    fn limit(&self) -> u64 {
        self.count.unwrap_or(DEFAULT_PAGE_SIZE)
    }

    fn offset(&self) -> u64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }
}

impl Model {
    /// Returns paginated published videos with optional filters.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_paginated(
        db: &DatabaseConnection,
        params: &VideosParams,
    ) -> Result<Vec<Self>, DbErr> {
        let mut select = Entity::find().filter(videos::Column::Published.eq(true));

        if let Some(channel_id) = params.channel_id {
            select = select.filter(videos::Column::ChannelId.eq(channel_id));
        }

        if let Some(ref q) = params.query {
            select = select.filter(videos::Column::Title.contains(q.as_str()));
        }

        if let Some(ref since) = params.since {
            if let Ok(date) = NaiveDate::parse_from_str(since, "%Y-%m-%d") {
                if let Some(naive) = date.and_hms_opt(0, 0, 0) {
                    let dt = naive.and_utc().fixed_offset();
                    select = select.filter(videos::Column::PublishedAt.gte(dt));
                }
            }
        }

        if let Some(ref until) = params.until {
            if let Ok(date) = NaiveDate::parse_from_str(until, "%Y-%m-%d") {
                if let Some(naive) = date.and_hms_opt(23, 59, 59) {
                    let dt = naive.and_utc().fixed_offset();
                    select = select.filter(videos::Column::PublishedAt.lte(dt));
                }
            }
        }

        if params.only_song_lives.unwrap_or(0) >= 1 {
            let subquery = Query::select()
                .expr(Expr::val(1i32))
                .from(song_items::Entity)
                .cond_where(
                    Condition::all()
                        .add(
                            Expr::col((song_items::Entity, song_items::Column::VideoId))
                                .eq(Expr::col((videos::Entity, videos::Column::Id))),
                        )
                        .add(
                            Expr::col((song_items::Entity, song_items::Column::LatestDiffId))
                                .is_not_null(),
                        ),
                )
                .to_owned();
            select = select.filter(Expr::exists(subquery));
        }

        select
            .order_by_desc(videos::Column::PublishedAt)
            .limit(params.limit())
            .offset(params.offset())
            .all(db)
            .await
    }

    /// Returns paginated videos for admin (no published filter).
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_paginated_admin(
        db: &DatabaseConnection,
        params: &VideosAdminParams,
    ) -> Result<Vec<Self>, DbErr> {
        let mut select = Entity::find();
        if let Some(channel_id) = params.channel_id {
            select = select.filter(videos::Column::ChannelId.eq(channel_id));
        }
        if params.only_song_lives.unwrap_or(false) {
            const KEYWORDS: &[&str] = &[
                "歌枠",
                "うたわく",
                "歌ってみた",
                "弾き語り",
                "歌配信",
                "カラオケ",
                "karaoke",
                "singing",
                "song",
            ];
            let cond = KEYWORDS.iter().fold(Condition::any(), |c, kw| {
                c.add(videos::Column::Title.contains(*kw))
            });
            select = select.filter(cond);
        }
        select
            .order_by_desc(videos::Column::PublishedAt)
            .limit(params.limit())
            .offset(params.offset())
            .all(db)
            .await
    }

    /// Finds a video by primary key.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Self>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    /// Returns all videos that are not yet completed.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_incomplete(db: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .filter(
                Condition::all()
                    .add(videos::Column::Status.ne(STATUS_COMPLETED))
                    .add(videos::Column::Status.ne(STATUS_COMMENTS_DISABLED)),
            )
            .all(db)
            .await
    }

    /// Updates the status of a video by primary key.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: i32,
    ) -> Result<Option<Self>, DbErr> {
        let Some(video) = Entity::find_by_id(id).one(db).await? else {
            return Ok(None);
        };
        let mut active: ActiveModel = video.into();
        active.status = ActiveValue::set(status);
        let model = active.update(db).await?;
        Ok(Some(model))
    }
}

impl ActiveModel {
    /// Creates and inserts a new video.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn create_from_params(
        db: &DatabaseConnection,
        params: CreateVideoParams,
    ) -> Result<Model, DbErr> {
        Self {
            video_id: ActiveValue::set(params.video_id),
            channel_id: ActiveValue::set(params.channel_id),
            title: ActiveValue::set(params.title),
            response_json: ActiveValue::set(params.response_json),
            kind: ActiveValue::set(0),
            status: ActiveValue::set(STATUS_FETCHED),
            published: ActiveValue::set(false),
            published_at: ActiveValue::set(params.published_at),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// Updates a video from admin params.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn update_from_params(
        db: &DatabaseConnection,
        id: i32,
        params: UpdateVideoParams,
    ) -> Result<Option<Model>, DbErr> {
        let Some(video) = Entity::find_by_id(id).one(db).await? else {
            return Ok(None);
        };
        let mut active: Self = video.into();
        if let Some(title) = params.title {
            active.title = ActiveValue::set(title);
        }
        if let Some(published) = params.published {
            active.published = ActiveValue::set(published);
        }
        if let Some(kind) = params.kind {
            active.kind = ActiveValue::set(kind);
        }
        if let Some(status) = params.status {
            active.status = ActiveValue::set(status);
        }
        let model = active.update(db).await?;
        Ok(Some(model))
    }
}
