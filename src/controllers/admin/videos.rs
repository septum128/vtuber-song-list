use crate::{
    controllers::admin::require_admin,
    models::{
        _entities::videos as videos_entity,
        videos::{self, ActiveModel, CreateVideoParams, UpdateVideoParams, VideosAdminParams},
    },
    workers::{
        song_items_creator::{SongItemsCreatorWorker, SongItemsCreatorWorkerArgs},
        youtube_client::YouTubeClient,
    },
};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
struct ListQuery {
    channel_id: Option<i64>,
    only_song_lives: Option<bool>,
    page: Option<u64>,
    count: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    video_id: String,
    channel_id: i64,
}

#[derive(Debug, Deserialize)]
struct UpdateBody {
    title: Option<String>,
    published: Option<bool>,
    kind: Option<i32>,
    status: Option<i32>,
}

#[debug_handler]
async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let videos = videos::Model::find_paginated_admin(
        &ctx.db,
        &VideosAdminParams {
            channel_id: q.channel_id,
            only_song_lives: q.only_song_lives,
            page: q.page,
            count: q.count,
        },
    )
    .await?;
    let items: Vec<VideoResponse> = videos.into_iter().map(Into::into).collect();
    format::json(items)
}

#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(body): Json<CreateBody>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;

    let video_id = extract_video_id(&body.video_id);

    // Check for duplicates
    let existing = videos_entity::Entity::find()
        .filter(videos_entity::Column::VideoId.eq(&video_id))
        .one(&ctx.db)
        .await?;
    if existing.is_some() {
        return Err(loco_rs::Error::BadRequest(
            "この動画はすでに登録されています".to_string(),
        ));
    }

    let google_api_key =
        std::env::var("GOOGLE_API_KEY").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
    let youtube_client = YouTubeClient::new(google_api_key);

    let infos = youtube_client
        .fetch_video_info(&[video_id.as_str()])
        .await
        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

    let info = infos.into_iter().next().ok_or_else(|| {
        loco_rs::Error::BadRequest("YouTube動画が見つかりませんでした".to_string())
    })?;

    let published_at = chrono::DateTime::parse_from_rfc3339(&info.published_at)
        .unwrap_or_else(|_| chrono::Utc::now().fixed_offset());

    let video = ActiveModel::create_from_params(
        &ctx.db,
        CreateVideoParams {
            video_id: info.video_id,
            channel_id: body.channel_id,
            title: info.title,
            published_at,
            response_json: info.response_json,
        },
    )
    .await?;

    if let Err(e) = SongItemsCreatorWorker::perform_later(&ctx, SongItemsCreatorWorkerArgs {}).await
    {
        tracing::warn!("Failed to enqueue SongItemsCreatorWorker: {e}");
    }

    format::json(VideoResponse::from(video))
}

#[debug_handler]
async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateBody>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    ActiveModel::update_from_params(
        &ctx.db,
        id,
        UpdateVideoParams {
            title: body.title,
            published: body.published,
            kind: body.kind,
            status: body.status,
        },
    )
    .await?
    .map_or_else(not_found, |video| format::json(VideoResponse::from(video)))
}

fn extract_video_id(input: &str) -> String {
    // https://www.youtube.com/watch?v=VIDEO_ID
    if let Some(pos) = input.find("v=") {
        let rest = &input[pos + 2..];
        let end = rest
            .find(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
            .unwrap_or(rest.len());
        return rest[..end].to_string();
    }
    // https://youtu.be/VIDEO_ID
    if let Some(pos) = input.rfind('/') {
        let rest = &input[pos + 1..];
        if !rest.is_empty() && !rest.contains('.') {
            let end = rest
                .find(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
                .unwrap_or(rest.len());
            return rest[..end].to_string();
        }
    }
    input.trim().to_string()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/videos")
        .add("/", get(list))
        .add("/", post(create))
        .add("/{id}", patch(update))
}
