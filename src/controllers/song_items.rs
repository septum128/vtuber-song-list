use crate::models::song_items::{SongItemRow, SongItemsParams};
use axum::extract::Query;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct SongItemListQuery {
    channel_id: Option<i64>,
    video_id: Option<i64>,
    query: Option<String>,
    since: Option<String>,
    until: Option<String>,
    video_title: Option<String>,
    page: Option<u64>,
    count: Option<u64>,
}

#[derive(Serialize)]
struct DiffInfo {
    title: Option<String>,
    author: Option<String>,
    time: Option<String>,
}

#[derive(Serialize)]
struct VideoInfo {
    id: i32,
    video_id: String,
    title: String,
    channel_id: i64,
    channel_custom_name: String,
    kind: i32,
    published_at: sea_orm::prelude::DateTimeWithTimeZone,
}

#[derive(Serialize)]
struct SongItemResponse {
    id: i32,
    video_id: i64,
    latest_diff_id: Option<i32>,
    diff: DiffInfo,
    video: VideoInfo,
}

impl From<SongItemRow> for SongItemResponse {
    fn from(r: SongItemRow) -> Self {
        Self {
            id: r.id,
            video_id: r.video_id,
            latest_diff_id: r.latest_diff_id,
            diff: DiffInfo {
                title: r.diff_title,
                author: r.diff_author,
                time: r.diff_time,
            },
            video: VideoInfo {
                id: r.v_id,
                video_id: r.v_video_id,
                title: r.v_title,
                channel_id: r.v_channel_id,
                channel_custom_name: r.v_channel_custom_name,
                kind: r.v_kind,
                published_at: r.v_published_at,
            },
        }
    }
}

#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Query(q): Query<SongItemListQuery>,
) -> Result<Response> {
    let params = SongItemsParams {
        channel_id: q.channel_id,
        video_id: q.video_id,
        query: q.query,
        since: q.since,
        until: q.until,
        video_title: q.video_title,
        page: q.page,
        count: q.count,
    };
    let rows = SongItemRow::find_paginated(&ctx.db, &params).await?;
    let items: Vec<SongItemResponse> = rows.into_iter().map(Into::into).collect();
    format::json(items)
}

#[debug_handler]
async fn get_one(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response> {
    SongItemRow::find_by_id(&ctx.db, id)
        .await?
        .map_or_else(not_found, |row| format::json(SongItemResponse::from(row)))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/song_items")
        .add("/", get(list))
        .add("/{id}", get(get_one))
}
