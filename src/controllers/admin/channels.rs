use crate::{
    controllers::admin::require_admin,
    models::channels::{self, ActiveModel, CreateChannelParams, UpdateChannelParams},
};
use axum::http::StatusCode;
use loco_rs::prelude::*;
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

    ActiveModel::update_from_params(
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
    .map_or_else(not_found, |channel| {
        format::json(ChannelResponse::from(channel))
    })
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
