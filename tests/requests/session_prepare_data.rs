use axum::http::{HeaderName, HeaderValue};
use loco_rs::{app::AppContext, TestServer};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use vtuber_song_list::models::_entities::{channels, users, videos};
use vtuber_song_list::models::song_items::{ActiveModel as SongItemActive, CreateSongItemParams};

#[derive(Debug, Deserialize)]
pub struct SessionResponse {
    pub token: String,
}

pub struct LoggedInUser {
    pub name: String,
    pub token: String,
}

/// Registers a user via `/api/user` and returns the token.
pub async fn register_and_login(request: &TestServer, name: &str, password: &str) -> LoggedInUser {
    let payload = serde_json::json!({
        "name": name,
        "password": password,
        "password_confirmation": password,
    });
    let res = request.post("/api/user").json(&payload).await;
    let body: SessionResponse = serde_json::from_str(&res.text()).unwrap();
    LoggedInUser {
        name: name.to_string(),
        token: body.token,
    }
}

/// Promotes an existing user to admin (kind = 10).
pub async fn make_admin(ctx: &AppContext, name: &str) {
    let email = format!("{}@local", name);
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&email))
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    let mut active: users::ActiveModel = user.into();
    active.kind = ActiveValue::set(10);
    active.update(&ctx.db).await.unwrap();
}

/// Logs in an existing user via `/api/session` and returns the token.
pub async fn login(request: &TestServer, name: &str, password: &str) -> String {
    let res = request
        .post("/api/session")
        .json(&serde_json::json!({ "name": name, "password": password }))
        .await;
    let body: SessionResponse = serde_json::from_str(&res.text()).unwrap();
    body.token
}

/// Returns a Bearer Authorization header tuple.
pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let value = HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
    (HeaderName::from_static("authorization"), value)
}

/// Inserts a published channel directly into the DB.
pub async fn create_channel(ctx: &AppContext) -> channels::Model {
    channels::ActiveModel {
        channel_id: ActiveValue::set("UC_test_channel_001".to_string()),
        name: ActiveValue::set(Some("Test Channel".to_string())),
        custom_name: ActiveValue::set("Test Channel".to_string()),
        twitter_id: ActiveValue::set(None),
        kind: ActiveValue::set(100), // KIND_PUBLISHED
        status: ActiveValue::set(0),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap()
}

/// Inserts a hidden (non-published) channel directly into the DB.
pub async fn create_hidden_channel(ctx: &AppContext) -> channels::Model {
    channels::ActiveModel {
        channel_id: ActiveValue::set("UC_hidden_channel_001".to_string()),
        name: ActiveValue::set(Some("Hidden Channel".to_string())),
        custom_name: ActiveValue::set("Hidden Channel".to_string()),
        twitter_id: ActiveValue::set(None),
        kind: ActiveValue::set(0), // KIND_HIDDEN
        status: ActiveValue::set(0),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap()
}

/// Inserts a published video directly into the DB.
pub async fn create_video(ctx: &AppContext, channel_id: i64) -> videos::Model {
    videos::ActiveModel {
        channel_id: ActiveValue::set(channel_id),
        video_id: ActiveValue::set("test_video_abc123".to_string()),
        title: ActiveValue::set("Test Video".to_string()),
        response_json: ActiveValue::set(serde_json::json!({})),
        kind: ActiveValue::set(10),   // LIVE
        status: ActiveValue::set(40), // COMPLETED
        published: ActiveValue::set(true),
        published_at: ActiveValue::set(chrono::Utc::now().fixed_offset()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap()
}

/// Creates a song_item with an approved diff directly in the DB.
pub async fn create_song_item(
    ctx: &AppContext,
    video_id: i64,
) -> vtuber_song_list::models::_entities::song_items::Model {
    SongItemActive::create_with_diff(
        &ctx.db,
        CreateSongItemParams {
            video_id,
            time: Some("00:01:00".to_string()),
            title: Some("Test Song".to_string()),
            author: Some("Test Artist".to_string()),
        },
    )
    .await
    .unwrap()
}
