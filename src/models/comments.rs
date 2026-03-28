use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

pub use super::_entities::comments::{self, ActiveModel, Entity, Model};

pub const STATUS_READY: i32 = 0;
pub const STATUS_FETCHED: i32 = 10;
pub const STATUS_COMPLETED: i32 = 20;

// Minimum number of timestamp matches required to consider a comment a setlist.
const SETLIST_MIN_TIMESTAMPS: usize = 3;

impl Model {
    /// Returns `true` if the comment contains enough timestamps to be considered a setlist.
    ///
    /// # Panics
    /// Panics if the built-in static timestamp regex fails to compile (should never occur).
    #[must_use]
    pub fn is_setlist(&self) -> bool {
        // Matches patterns like: HH:MM:SS, H:MM:SS, MM:SS, M:SS
        let re = Regex::new(r"([0-9]{2}:)?[0-9]?[0-9]:[0-9]{2}").expect("static regex is valid");
        re.find_iter(&self.content).count() >= SETLIST_MIN_TIMESTAMPS
    }

    /// Returns all comments for a video, ordered by creation time.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_by_video(db: &DatabaseConnection, video_id: i64) -> Result<Vec<Self>, DbErr> {
        Entity::find()
            .filter(comments::Column::VideoId.eq(video_id))
            .all(db)
            .await
    }

    /// Marks a comment as completed.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn mark_completed(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
        let Some(comment) = Entity::find_by_id(id).one(db).await? else {
            return Ok(());
        };
        let mut active: ActiveModel = comment.into();
        active.status = ActiveValue::set(STATUS_COMPLETED);
        active.update(db).await?;
        Ok(())
    }
}

pub struct UpsertCommentParams {
    pub comment_id: String,
    pub video_id: i64,
    pub author: String,
    pub content: String,
    pub response_json: serde_json::Value,
}

impl ActiveModel {
    /// Inserts a new comment, or updates an existing one if `comment_id` already exists.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn upsert(
        db: &DatabaseConnection,
        params: UpsertCommentParams,
    ) -> Result<Model, DbErr> {
        let existing = Entity::find()
            .filter(comments::Column::CommentId.eq(&params.comment_id))
            .one(db)
            .await?;

        if let Some(existing) = existing {
            let mut active: Self = existing.into();
            active.author = ActiveValue::set(params.author);
            active.content = ActiveValue::set(params.content);
            active.response_json = ActiveValue::set(params.response_json);
            active.update(db).await
        } else {
            Self {
                comment_id: ActiveValue::set(params.comment_id),
                video_id: ActiveValue::set(params.video_id),
                author: ActiveValue::set(params.author),
                content: ActiveValue::set(params.content),
                response_json: ActiveValue::set(params.response_json),
                status: ActiveValue::set(STATUS_READY),
                ..Default::default()
            }
            .insert(db)
            .await
        }
    }
}
