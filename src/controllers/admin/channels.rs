use crate::{
    controllers::admin::require_admin,
    models::_entities::channels as channel_entity,
    models::channels::{self, ActiveModel, CreateChannelParams, UpdateChannelParams},
    workers::youtube_client::YouTubeClient,
};
use axum::http::StatusCode;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChannelResponse {
    id: i32,
    channel_id: String,
    name: Option<String>,
    custom_name: String,
    twitter_id: Option<String>,
    kind: i32,
    status: i32,
    icon_url: Option<String>,
}

impl From<channels::Model> for ChannelResponse {
    fn from(m: channels::Model) -> Self {
        Self {
            id: m.id,
            channel_id: m.channel_id,
            name: m.name,
            custom_name: m.custom_name,
            twitter_id: m.twitter_id,
            kind: m.kind,
            status: m.status,
            icon_url: m.icon_url,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    channel_id: String,
    name: Option<String>,
    custom_name: String,
    twitter_id: Option<String>,
    kind: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct UpdateBody {
    name: Option<serde_json::Value>,
    custom_name: Option<String>,
    twitter_id: Option<serde_json::Value>,
    kind: Option<i32>,
    fetch_icon: Option<bool>,
}

async fn try_fetch_icon(channel_id: &str) -> Option<String> {
    let Ok(api_key) = std::env::var("GOOGLE_API_KEY") else {
        tracing::warn!("GOOGLE_API_KEY が設定されていないためアイコン取得をスキップします");
        return None;
    };
    let client = YouTubeClient::new(api_key);
    match client.fetch_channel_icon(channel_id).await {
        Ok(Some(url)) => Some(url),
        Ok(None) => {
            tracing::warn!(channel_id, "チャンネルアイコンが見つかりませんでした");
            None
        }
        Err(e) => {
            tracing::warn!(
                err = e.to_string(),
                channel_id,
                "アイコン取得に失敗しました"
            );
            None
        }
    }
}

#[debug_handler]
async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let channels = channels::Model::find_all_admin(&ctx.db).await?;
    let items: Vec<ChannelResponse> = channels.into_iter().map(Into::into).collect();
    format::json(items)
}

#[debug_handler]
async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(body): Json<CreateBody>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let channel_id_for_icon = body.channel_id.clone();
    let channel = ActiveModel::create_from_params(
        &ctx.db,
        CreateChannelParams {
            channel_id: body.channel_id,
            name: body.name,
            custom_name: body.custom_name,
            twitter_id: body.twitter_id,
            kind: body.kind.unwrap_or(channels::KIND_HIDDEN),
        },
    )
    .await
    .map_err(|e| {
        tracing::error!(err = e.to_string(), "failed to create channel");
        Error::BadRequest("チャンネルの作成に失敗しました".into())
    })?;

    let channel = if let Some(url) = try_fetch_icon(&channel_id_for_icon).await {
        let mut active: channel_entity::ActiveModel = channel.into();
        active.icon_url = ActiveValue::set(Some(url));
        active.update(&ctx.db).await.map_err(|e| {
            tracing::error!(
                err = e.to_string(),
                "failed to update icon_url after create"
            );
            Error::BadRequest("アイコンURLの更新に失敗しました".into())
        })?
    } else {
        channel
    };

    let body = format::json(ChannelResponse::from(channel))?;
    Ok((StatusCode::CREATED, body).into_response())
}

#[debug_handler]
async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateBody>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;

    let fetch_icon = body.fetch_icon == Some(true);

    // Resolve nullable fields: JSON null → Some(None), missing → None
    let name = body.name.map(|v| {
        if v.is_null() {
            None
        } else {
            v.as_str().map(ToString::to_string)
        }
    });
    let twitter_id = body.twitter_id.map(|v| {
        if v.is_null() {
            None
        } else {
            v.as_str().map(ToString::to_string)
        }
    });

    let Some(channel) = ActiveModel::update_from_params(
        &ctx.db,
        id,
        UpdateChannelParams {
            name,
            custom_name: body.custom_name,
            twitter_id,
            kind: body.kind,
        },
    )
    .await?
    else {
        return not_found();
    };

    let channel = if fetch_icon {
        if let Some(url) = try_fetch_icon(&channel.channel_id).await {
            let mut active: channel_entity::ActiveModel = channel.into();
            active.icon_url = ActiveValue::set(Some(url));
            active.update(&ctx.db).await.map_err(|e| {
                tracing::error!(err = e.to_string(), "failed to update icon_url");
                Error::BadRequest("アイコンURLの更新に失敗しました".into())
            })?
        } else {
            channel
        }
    } else {
        channel
    };

    format::json(ChannelResponse::from(channel))
}

#[debug_handler]
async fn delete(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    require_admin(&auth, &ctx).await?;
    let deleted = channels::Model::delete_by_id(&ctx.db, id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        not_found()
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/channels")
        .add("/", get(list))
        .add("/", post(create))
        .add("/{id}", patch(update))
        .add("/{id}", axum::routing::delete(delete))
}
