use std::fmt::Write as _;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait,
    FromQueryResult, QueryFilter, Statement,
};
use serde::Serialize;

pub use super::_entities::song_items::{self, ActiveModel, Entity, Model};

pub const DEFAULT_PAGE_SIZE: u64 = 20;

pub struct SongItemsParams {
    pub channel_id: Option<i64>,
    pub video_id: Option<i64>,
    pub query: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub video_title: Option<String>,
    pub page: Option<u64>,
    pub count: Option<u64>,
}

pub struct CreateSongItemParams {
    pub video_id: i64,
    pub time: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
}

impl SongItemsParams {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn limit(&self) -> i64 {
        self.count.unwrap_or(DEFAULT_PAGE_SIZE) as i64
    }

    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        ((page - 1) * self.count.unwrap_or(DEFAULT_PAGE_SIZE)) as i64
    }
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct SongItemRow {
    pub id: i32,
    pub video_id: i64,
    pub latest_diff_id: Option<i32>,
    // video fields
    pub v_id: i32,
    pub v_video_id: String,
    pub v_title: String,
    pub v_channel_id: i64,
    pub v_kind: i32,
    pub v_published_at: sea_orm::prelude::DateTimeWithTimeZone,
    // diff fields
    pub diff_title: Option<String>,
    pub diff_author: Option<String>,
    pub diff_time: Option<String>,
}

/// Builds a parameterized SQL query for `song_items` with filters.
fn build_query(params: &SongItemsParams) -> (String, Vec<sea_orm::Value>) {
    let base = r"
        SELECT
            si.id, si.video_id, si.latest_diff_id,
            v.id        AS v_id,
            v.video_id  AS v_video_id,
            v.title     AS v_title,
            v.channel_id AS v_channel_id,
            v.kind      AS v_kind,
            v.published_at AS v_published_at,
            sd.title    AS diff_title,
            sd.author   AS diff_author,
            sd.time     AS diff_time
        FROM song_items si
        INNER JOIN videos v ON si.video_id = v.id
        LEFT  JOIN song_diffs sd ON si.latest_diff_id = sd.id
        WHERE si.latest_diff_id IS NOT NULL
          AND v.published = true
    ";

    let mut sql = base.to_string();
    let mut values: Vec<sea_orm::Value> = Vec::new();
    let mut idx = 1usize;

    macro_rules! push_cond {
        ($cond:expr, $val:expr) => {{
            let _ = write!(sql, " AND {}", $cond.replace("{}", &format!("${idx}")));
            values.push($val);
            idx += 1;
        }};
    }

    if let Some(channel_id) = params.channel_id {
        push_cond!("v.channel_id = {}", channel_id.into());
    }
    if let Some(video_id) = params.video_id {
        push_cond!("si.video_id = {}", video_id.into());
    }
    if let Some(ref q) = params.query {
        let pattern = format!("%{q}%");
        let _ = write!(
            sql,
            " AND (sd.title ILIKE ${idx} OR sd.author ILIKE ${idx})"
        );
        values.push(pattern.into());
        idx += 1;
    }
    if let Some(ref since) = params.since {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(since, "%Y-%m-%d") {
            if let Some(naive) = date.and_hms_opt(0, 0, 0) {
                let dt = naive.and_utc().fixed_offset();
                push_cond!("v.published_at >= {}", dt.into());
            }
        }
    }
    if let Some(ref until) = params.until {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(until, "%Y-%m-%d") {
            if let Some(naive) = date.and_hms_opt(23, 59, 59) {
                let dt = naive.and_utc().fixed_offset();
                push_cond!("v.published_at <= {}", dt.into());
            }
        }
    }
    if let Some(ref video_title) = params.video_title {
        let pattern = format!("%{video_title}%");
        push_cond!("v.title ILIKE {}", pattern.into());
    }

    let _ = write!(
        sql,
        " ORDER BY v.published_at DESC, sd.time ASC NULLS LAST LIMIT ${} OFFSET ${}",
        idx,
        idx + 1
    );
    values.push(params.limit().into());
    values.push(params.offset().into());

    (sql, values)
}

impl SongItemRow {
    /// Returns paginated song items with video and diff info.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_paginated(
        db: &DatabaseConnection,
        params: &SongItemsParams,
    ) -> Result<Vec<Self>, DbErr> {
        let (sql, values) = build_query(params);
        Self::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            values,
        ))
        .all(db)
        .await
    }

    /// Returns a single song item with video and diff info.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Self>, DbErr> {
        let sql = r"
            SELECT
                si.id, si.video_id, si.latest_diff_id,
                v.id        AS v_id,
                v.video_id  AS v_video_id,
                v.title     AS v_title,
                v.channel_id AS v_channel_id,
                v.kind      AS v_kind,
                v.published_at AS v_published_at,
                sd.title    AS diff_title,
                sd.author   AS diff_author,
                sd.time     AS diff_time
            FROM song_items si
            INNER JOIN videos v ON si.video_id = v.id
            LEFT  JOIN song_diffs sd ON si.latest_diff_id = sd.id
            WHERE si.id = $1
        ";
        Self::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            [id.into()],
        ))
        .one(db)
        .await
    }

    /// Returns paginated song items for admin (no published filter).
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn find_paginated_admin(
        db: &DatabaseConnection,
        video_id: i64,
        page: u64,
        count: u64,
    ) -> Result<Vec<Self>, DbErr> {
        #[allow(clippy::cast_possible_wrap)]
        let limit = count as i64;
        #[allow(clippy::cast_possible_wrap)]
        let offset = ((page.max(1) - 1) * count) as i64;

        let sql = r"
            SELECT
                si.id, si.video_id, si.latest_diff_id,
                v.id        AS v_id,
                v.video_id  AS v_video_id,
                v.title     AS v_title,
                v.channel_id AS v_channel_id,
                v.kind      AS v_kind,
                v.published_at AS v_published_at,
                sd.title    AS diff_title,
                sd.author   AS diff_author,
                sd.time     AS diff_time
            FROM song_items si
            INNER JOIN videos v ON si.video_id = v.id
            LEFT  JOIN song_diffs sd ON si.latest_diff_id = sd.id
            WHERE si.video_id = $1
            ORDER BY sd.time ASC NULLS LAST
            LIMIT $2 OFFSET $3
        ";
        Self::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            [video_id.into(), limit.into(), offset.into()],
        ))
        .all(db)
        .await
    }
}

impl Model {
    /// Deletes a song item and its associated song diffs.
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<bool, DbErr> {
        use super::_entities::song_diffs;

        // Null out latest_diff_id to avoid FK conflict.
        if let Some(item) = Entity::find_by_id(id).one(db).await? {
            let mut active: ActiveModel = item.into();
            active.latest_diff_id = ActiveValue::set(None);
            active.update(db).await?;
        } else {
            return Ok(false);
        }

        // Delete related song_diffs.
        song_diffs::Entity::delete_many()
            .filter(song_diffs::Column::SongItemId.eq(i64::from(id)))
            .exec(db)
            .await?;

        // Delete the song_item.
        let result = Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }
}

impl ActiveModel {
    /// Creates a song item with an approved song diff (admin use).
    ///
    /// # Errors
    /// Returns `DbErr` on database failure.
    pub async fn create_with_diff(
        db: &DatabaseConnection,
        params: CreateSongItemParams,
    ) -> Result<Model, DbErr> {
        use super::_entities::song_diffs;
        use super::song_diffs::{KIND_MANUAL, STATUS_APPROVED};

        // Create the song_item first (latest_diff_id = NULL).
        let item = Self {
            video_id: ActiveValue::set(params.video_id),
            latest_diff_id: ActiveValue::set(None),
            ..Default::default()
        }
        .insert(db)
        .await?;

        // Create the approved diff.
        let diff = song_diffs::ActiveModel {
            song_item_id: ActiveValue::set(i64::from(item.id)),
            time: ActiveValue::set(params.time),
            title: ActiveValue::set(params.title),
            author: ActiveValue::set(params.author),
            status: ActiveValue::set(STATUS_APPROVED),
            kind: ActiveValue::set(KIND_MANUAL),
            ..Default::default()
        }
        .insert(db)
        .await?;

        // Update latest_diff_id.
        let mut active: Self = item.into();
        active.latest_diff_id = ActiveValue::set(Some(diff.id));
        let model = active.update(db).await?;
        Ok(model)
    }
}
