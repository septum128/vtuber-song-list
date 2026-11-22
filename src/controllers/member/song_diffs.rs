use crate::models::{
    _entities::users,
    song_diffs::{self, ActiveModel, CreateSongDiffParams},
    song_items::SongItemRow,
};
use axum::http::StatusCode;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

// kind=admin
const USER_KIND_ADMIN: i32 = 10;

#[derive(Debug, Deserialize)]
struct CreateSongDiffBody {
    time: Option<String>,
    title: Option<String>,
    author: Option<String>,
}

#[derive(Serialize)]
struct SongDiffResponse {
    id: i64,
    song_item_id: i64,
    made_by_id: Option<i64>,
    time: Option<String>,
    title: Option<String>,
    author: Option<String>,
    status: i32,
    kind: i32,
    created_at: sea_orm::prelude::DateTimeWithTimeZone,
}

impl From<song_diffs::Model> for SongDiffResponse {
    fn from(m: song_diffs::Model) -> Self {
        Self {
            id: m.id,
            song_item_id: m.song_item_id,
            made_by_id: m.made_by_id,
            time: m.time,
            title: m.title,
            author: m.author,
            status: m.status,
            kind: m.kind,
            created_at: m.created_at,
        }
    }
}

/// Returns 404 unless the song item exists and the user may see it.
/// Non-admin users may only access song items of published videos.
async fn ensure_song_item_visible(
    ctx: &AppContext,
    user: &users::Model,
    song_item_id: i64,
) -> Result<()> {
    if user.kind == USER_KIND_ADMIN {
        return Ok(());
    }
    if SongItemRow::find_published_by_id(&ctx.db, song_item_id)
        .await?
        .is_none()
    {
        return not_found();
    }
    Ok(())
}

#[debug_handler]
async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(song_item_id): Path<i64>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    ensure_song_item_visible(&ctx, &user, song_item_id).await?;

    let diffs = song_diffs::Model::find_by_song_item(&ctx.db, song_item_id).await?;
    let items: Vec<SongDiffResponse> = diffs.into_iter().map(Into::into).collect();
    format::json(items)
}

#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(song_item_id): Path<i64>,
    Json(body): Json<CreateSongDiffBody>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    ensure_song_item_visible(&ctx, &user, song_item_id).await?;

    let diff = ActiveModel::create_from_params(
        &ctx.db,
        CreateSongDiffParams {
            song_item_id,
            made_by_id: user.id,
            time: body.time,
            title: body.title,
            author: body.author,
            is_admin: user.kind == USER_KIND_ADMIN,
        },
    )
    .await
    .map_err(|e| {
        tracing::error!(err = e.to_string(), "failed to create song diff");
        Error::BadRequest("歌情報を修正できませんでした。".into())
    })?;

    let body = format::json(SongDiffResponse::from(diff))?;
    Ok((StatusCode::CREATED, body).into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/member/song_items/{song_item_id}/song_diffs")
        .add("/", get(list))
        .add("/", post(create))
}
