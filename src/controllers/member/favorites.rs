use crate::models::{
    _entities::users,
    favorites::{self, ActiveModel},
    song_items::SongItemRow,
};
use axum::http::StatusCode;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct CreateFavoriteBody {
    song_item_id: i64,
}

#[derive(Serialize)]
struct FavoriteIdsResponse {
    song_item_ids: Vec<i64>,
}

// Reuse the same response shape as /api/song_items
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
                kind: r.v_kind,
                published_at: r.v_published_at,
            },
        }
    }
}

/// GET /api/member/favorites/ids — ユーザーのお気に入り曲IDリスト
#[debug_handler]
async fn list_ids(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let ids = favorites::Model::find_song_item_ids(&ctx.db, user.id.into()).await?;
    format::json(FavoriteIdsResponse { song_item_ids: ids })
}

/// GET /api/member/favorites — ユーザーのお気に入り曲詳細一覧
#[debug_handler]
async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let rows = favorites::Model::find_song_items(&ctx.db, user.id.into()).await?;
    let items: Vec<SongItemResponse> = rows.into_iter().map(Into::into).collect();
    format::json(items)
}

/// POST /api/member/favorites — お気に入り追加
#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(body): Json<CreateFavoriteBody>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 既にお気に入り済みなら 200 を返す
    if favorites::Model::find_by_user_and_song_item(&ctx.db, user.id.into(), body.song_item_id)
        .await?
        .is_some()
    {
        return format::json(serde_json::json!({ "song_item_id": body.song_item_id }));
    }

    ActiveModel::create(&ctx.db, user.id.into(), body.song_item_id)
        .await
        .map_err(|e| {
            tracing::error!(err = e.to_string(), "failed to create favorite");
            Error::BadRequest("お気に入りの追加に失敗しました".into())
        })?;

    let body = format::json(serde_json::json!({ "song_item_id": body.song_item_id }))?;
    Ok((StatusCode::CREATED, body).into_response())
}

/// DELETE `/api/member/favorites/:song_item_id` — お気に入り削除
#[debug_handler]
async fn destroy(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(song_item_id): Path<i64>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if let Some(fav) =
        favorites::Model::find_by_user_and_song_item(&ctx.db, user.id.into(), song_item_id).await?
    {
        use sea_orm::EntityTrait;
        favorites::Entity::delete_by_id(fav.id)
            .exec(&ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(err = e.to_string(), "failed to delete favorite");
                Error::BadRequest("お気に入りの削除に失敗しました".into())
            })?;
    }

    format::json(serde_json::json!({ "song_item_id": song_item_id }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/member/favorites")
        .add("/ids", get(list_ids))
        .add("/", get(list))
        .add("/", post(create))
        .add("/{song_item_id}", delete(destroy))
}
