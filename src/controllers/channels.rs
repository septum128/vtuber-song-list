use crate::models::channels;
use loco_rs::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct ChannelResponse {
    id: i32,
    channel_id: String,
    name: Option<String>,
    custom_name: String,
    twitter_id: Option<String>,
    icon_url: Option<String>,
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
            icon_url: m.icon_url,
            kind: m.kind,
            status: m.status,
        }
    }
}

#[debug_handler]
async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let channels = channels::Model::find_all_published(&ctx.db).await?;
    let items: Vec<ChannelResponse> = channels.into_iter().map(Into::into).collect();
    format::json(items)
}

#[debug_handler]
async fn get_one(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response> {
    channels::Model::find_by_id(&ctx.db, id)
        .await?
        .map_or_else(not_found, |channel| {
            format::json(ChannelResponse::from(channel))
        })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/channels")
        .add("/", get(list))
        .add("/{id}", get(get_one))
}
