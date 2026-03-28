use crate::models::videos::{self, VideosParams};
use axum::extract::Query;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct VideoListQuery {
    channel_id: Option<i64>,
    query: Option<String>,
    since: Option<String>,
    until: Option<String>,
    only_song_lives: Option<i64>,
    page: Option<u64>,
    count: Option<u64>,
}

#[derive(Serialize)]
struct VideoResponse {
    id: i32,
    channel_id: i64,
    video_id: String,
    title: String,
    kind: i32,
    status: i32,
    published: bool,
    published_at: sea_orm::prelude::DateTimeWithTimeZone,
}

impl From<videos::Model> for VideoResponse {
    fn from(m: videos::Model) -> Self {
        Self {
            id: m.id,
            channel_id: m.channel_id,
            video_id: m.video_id,
            title: m.title,
            kind: m.kind,
            status: m.status,
            published: m.published,
            published_at: m.published_at,
        }
    }
}

#[debug_handler]
async fn list(State(ctx): State<AppContext>, Query(q): Query<VideoListQuery>) -> Result<Response> {
    let params = VideosParams {
        channel_id: q.channel_id,
        query: q.query,
        since: q.since,
        until: q.until,
        only_song_lives: q.only_song_lives,
        page: q.page,
        count: q.count,
    };
    let videos = videos::Model::find_paginated(&ctx.db, &params).await?;
    let items: Vec<VideoResponse> = videos.into_iter().map(Into::into).collect();
    format::json(items)
}

#[debug_handler]
async fn get_one(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response> {
    videos::Model::find_by_id(&ctx.db, id)
        .await?
        .map_or_else(not_found, |video| format::json(VideoResponse::from(video)))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/videos")
        .add("/", get(list))
        .add("/{id}", get(get_one))
}
