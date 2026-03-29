use crate::{
    controllers::admin::require_admin,
    models::song_items::{self, ActiveModel, CreateSongItemParams, SongItemRow},
};
use axum::http::StatusCode;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SongItemResponse {
    id: i32,
    video_id: i64,
    latest_diff_id: Option<i32>,
    video: VideoInfo,
    diff: DiffInfo,
}

#[derive(Serialize)]
struct VideoInfo {
    id: i32,
    video_id: String,
    title: String,
    channel_id: i64,
    kind: i32,
    published_at: sea_orm::prelude::DateTimeWithTimeZone,
}

#[derive(Serialize)]
struct DiffInfo {
    time: Option<String>,
    title: Option<String>,
    author: Option<String>,
}

impl From<SongItemRow> for SongItemResponse {
    fn from(r: SongItemRow) -> Self {
        Self {
            id: r.id,
            video_id: r.video_id,
            latest_diff_id: r.latest_diff_id,
            video: VideoInfo {
                id: r.v_id,
                video_id: r.v_video_id,
                title: r.v_title,
                channel_id: r.v_channel_id,
                kind: r.v_kind,
                published_at: r.v_published_at,
            },
            diff: DiffInfo {
                time: r.diff_time,
                title: r.diff_title,
                author: r.diff_author,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    video_id: i64,
    page: Option<u64>,
    count: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    video_id: i64,
    time: Option<String>,
    title: Option<String>,
    author: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BulkItem {
    time: Option<String>,
    title: Option<String>,
    author: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BulkCreateBody {
    video_id: i64,
    items: Vec<BulkItem>,
}

#[debug_handler]
async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let items = SongItemRow::find_paginated_admin(
        &ctx.db,
        q.video_id,
        q.page.unwrap_or(1),
        q.count.unwrap_or(50),
    )
    .await?;
    let resp: Vec<SongItemResponse> = items.into_iter().map(Into::into).collect();
    format::json(resp)
}

#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(body): Json<CreateBody>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let item = ActiveModel::create_with_diff(
        &ctx.db,
        CreateSongItemParams {
            video_id: body.video_id,
            time: body.time,
            title: body.title,
            author: body.author,
        },
    )
    .await
    .map_err(|e| {
        tracing::error!(err = e.to_string(), "failed to create song item");
        Error::BadRequest("セトリの作成に失敗しました".into())
    })?;

    // Fetch the full row with video/diff info for response.
    let row = SongItemRow::find_by_id(&ctx.db, item.id)
        .await?
        .ok_or_else(|| Error::BadRequest("セトリの取得に失敗しました".into()))?;

    let body = format::json(SongItemResponse::from(row))?;
    Ok((StatusCode::CREATED, body).into_response())
}

#[debug_handler]
async fn bulk_create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(body): Json<BulkCreateBody>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;

    if body.items.is_empty() {
        return Err(Error::BadRequest("インポートするデータがありません".into()));
    }

    let mut created = Vec::new();
    for item in body.items {
        let song_item = ActiveModel::create_with_diff(
            &ctx.db,
            CreateSongItemParams {
                video_id: body.video_id,
                time: item.time,
                title: item.title,
                author: item.author,
            },
        )
        .await
        .map_err(|e| {
            tracing::error!(err = e.to_string(), "failed to bulk create song item");
            Error::BadRequest("セトリの作成に失敗しました".into())
        })?;
        created.push(song_item.id);
    }

    format::json(serde_json::json!({ "created": created.len() }))
}

#[debug_handler]
async fn delete(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let deleted = song_items::Model::delete_by_id(&ctx.db, id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        not_found()
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/song_items")
        .add("/", get(list))
        .add("/", post(create))
        .add("/bulk", post(bulk_create))
        .add("/{id}", axum::routing::delete(delete))
}
