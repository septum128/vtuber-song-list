use loco_rs::testing::prelude::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue;
use serial_test::serial;
use vtuber_song_list::app::App;
use vtuber_song_list::models::_entities::videos as videos_entity;

use crate::requests::session_prepare_data as pd;

async fn create_video_with(
    ctx: &loco_rs::app::AppContext,
    channel_id: i64,
    video_id: &str,
    published: bool,
) -> videos_entity::Model {
    videos_entity::ActiveModel {
        channel_id: ActiveValue::set(channel_id),
        video_id: ActiveValue::set(video_id.to_string()),
        title: ActiveValue::set(format!("Test Video {video_id}")),
        response_json: ActiveValue::set(serde_json::json!({})),
        kind: ActiveValue::set(10),
        status: ActiveValue::set(40),
        published: ActiveValue::set(published),
        published_at: ActiveValue::set(chrono::Utc::now().fixed_offset()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap()
}

#[tokio::test]
#[serial]
async fn admin_can_bulk_publish_videos() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let v1 = create_video_with(&ctx, i64::from(channel.id), "bulk_pub_1", false).await;
        let v2 = create_video_with(&ctx, i64::from(channel.id), "bulk_pub_2", false).await;

        let admin = pd::register_and_login(&request, "bulk_publish_admin", "password123").await;
        pd::make_admin(&ctx, "bulk_publish_admin").await;
        let token = pd::login(&request, "bulk_publish_admin", "password123").await;
        let (key, val) = pd::auth_header(&token);

        let res = request
            .post("/api/admin/videos/bulk_publish")
            .add_header(key, val)
            .json(&serde_json::json!({ "video_ids": [v1.id, v2.id], "published": true }))
            .await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["updated"], 2);

        drop(admin);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_can_bulk_unpublish_videos() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let v1 = create_video_with(&ctx, i64::from(channel.id), "bulk_unpub_1", true).await;
        let v2 = create_video_with(&ctx, i64::from(channel.id), "bulk_unpub_2", true).await;

        let admin = pd::register_and_login(&request, "bulk_unpublish_admin", "password123").await;
        pd::make_admin(&ctx, "bulk_unpublish_admin").await;
        let token = pd::login(&request, "bulk_unpublish_admin", "password123").await;
        let (key, val) = pd::auth_header(&token);

        let res = request
            .post("/api/admin/videos/bulk_publish")
            .add_header(key, val)
            .json(&serde_json::json!({ "video_ids": [v1.id, v2.id], "published": false }))
            .await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["updated"], 2);

        drop(admin);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn member_cannot_bulk_publish_videos() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let v1 = create_video_with(&ctx, i64::from(channel.id), "bulk_member_1", false).await;

        let member =
            pd::register_and_login(&request, "not_admin_bulk_publish", "password123").await;
        let (key, val) = pd::auth_header(&member.token);

        let res = request
            .post("/api/admin/videos/bulk_publish")
            .add_header(key, val)
            .json(&serde_json::json!({ "video_ids": [v1.id], "published": true }))
            .await;

        assert!(
            res.status_code() == 401 || res.status_code() == 403,
            "Non-admin should be rejected, got {}",
            res.status_code()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn bulk_publish_rejects_empty_ids() {
    request::<App, _, _>(|request, ctx| async move {
        let admin =
            pd::register_and_login(&request, "empty_bulk_publish_admin", "password123").await;
        pd::make_admin(&ctx, "empty_bulk_publish_admin").await;
        let token = pd::login(&request, "empty_bulk_publish_admin", "password123").await;
        let (key, val) = pd::auth_header(&token);

        let res = request
            .post("/api/admin/videos/bulk_publish")
            .add_header(key, val)
            .json(&serde_json::json!({ "video_ids": [], "published": true }))
            .await;

        assert_eq!(res.status_code(), 400);

        drop(admin);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_can_sort_videos_by_id() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let newer = create_video_with(&ctx, i64::from(channel.id), "sort_newer", false).await;
        // Registered later but streamed earlier (e.g. an archive fetched via API).
        let older = create_video_with(&ctx, i64::from(channel.id), "sort_older", false).await;
        let mut active: videos_entity::ActiveModel = older.clone().into();
        active.published_at =
            ActiveValue::set((chrono::Utc::now() - chrono::Duration::days(30)).fixed_offset());
        active.update(&ctx.db).await.unwrap();

        pd::register_and_login(&request, "sort_admin", "password123").await;
        pd::make_admin(&ctx, "sort_admin").await;
        let token = pd::login(&request, "sort_admin", "password123").await;

        let ids = |body: &str| -> Vec<i64> {
            let v: serde_json::Value = serde_json::from_str(body).unwrap();
            v.as_array()
                .unwrap()
                .iter()
                .map(|x| x["id"].as_i64().unwrap())
                .collect()
        };

        let (key, val) = pd::auth_header(&token);
        let res = request.get("/api/admin/videos").add_header(key, val).await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(
            ids(&res.text()),
            vec![i64::from(newer.id), i64::from(older.id)]
        );

        let (key, val) = pd::auth_header(&token);
        let res = request
            .get("/api/admin/videos?sort=id")
            .add_header(key, val)
            .await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(
            ids(&res.text()),
            vec![i64::from(older.id), i64::from(newer.id)]
        );
    })
    .await;
}
