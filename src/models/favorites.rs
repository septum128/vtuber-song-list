use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

pub use super::_entities::favorites::{self, ActiveModel, Entity, Model};
use crate::models::song_items::SongItemRow;

impl Model {
    /// Returns all favorited song item IDs for the given user.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_song_item_ids(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<i64>, DbErr> {
        let rows = Entity::find()
            .filter(favorites::Column::UserId.eq(user_id))
            .all(db)
            .await?;
        Ok(rows.into_iter().map(|r| r.song_item_id).collect())
    }

    /// Returns full song item details for all favorites of the given user.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_song_items(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<SongItemRow>, DbErr> {
        use sea_orm::{DbBackend, FromQueryResult, Statement};

        let sql = r"
            SELECT
                si.id, si.video_id, si.latest_diff_id,
                v.id            AS v_id,
                v.video_id      AS v_video_id,
                v.title         AS v_title,
                v.channel_id    AS v_channel_id,
                v.kind          AS v_kind,
                v.published_at  AS v_published_at,
                c.custom_name   AS v_channel_custom_name,
                sd.title        AS diff_title,
                sd.author       AS diff_author,
                sd.time         AS diff_time
            FROM favorites f
            INNER JOIN song_items si ON f.song_item_id = si.id
            INNER JOIN videos     v  ON si.video_id    = v.id
            INNER JOIN channels   c  ON v.channel_id   = c.id
            LEFT  JOIN song_diffs sd ON si.latest_diff_id = sd.id
            WHERE f.user_id = $1
              AND si.latest_diff_id IS NOT NULL
              AND v.published = true
            ORDER BY f.created_at DESC
        ";

        SongItemRow::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            [user_id.into()],
        ))
        .all(db)
        .await
    }

    /// Returns the favorite row if the user has favorited the song item.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_by_user_and_song_item(
        db: &DatabaseConnection,
        user_id: i64,
        song_item_id: i64,
    ) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(favorites::Column::UserId.eq(user_id))
            .filter(favorites::Column::SongItemId.eq(song_item_id))
            .one(db)
            .await
    }
}

impl ActiveModel {
    /// Creates a new favorite.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure or unique constraint violation.
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i64,
        song_item_id: i64,
    ) -> Result<Model, DbErr> {
        Self {
            user_id: ActiveValue::Set(user_id),
            song_item_id: ActiveValue::Set(song_item_id),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
