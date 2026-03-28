use crate::{
    controllers::admin::require_admin,
    models::song_diffs::{self, SongDiffsAdminParams},
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SongDiffResponse {
    id: i32,
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

#[derive(Debug, Deserialize)]
struct ListQuery {
    status: Option<i32>,
    page: Option<u64>,
    count: Option<u64>,
}

#[debug_handler]
async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let diffs = song_diffs::Model::find_all_admin(
        &ctx.db,
        &SongDiffsAdminParams {
            status: q.status,
            page: q.page,
            count: q.count,
        },
    )
    .await?;
    let items: Vec<SongDiffResponse> = diffs.into_iter().map(Into::into).collect();
    format::json(items)
}

#[debug_handler]
async fn approve(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    song_diffs::Model::approve(&ctx.db, id)
        .await?
        .map_or_else(not_found, |diff| format::json(SongDiffResponse::from(diff)))
}

#[debug_handler]
async fn reject(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    song_diffs::Model::reject(&ctx.db, id)
        .await?
        .map_or_else(not_found, |diff| format::json(SongDiffResponse::from(diff)))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/song_diffs")
        .add("/", get(list))
        .add("/{id}/approve", patch(approve))
        .add("/{id}/reject", patch(reject))
}
