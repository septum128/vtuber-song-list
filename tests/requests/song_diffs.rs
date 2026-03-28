use loco_rs::testing::prelude::*;
use serial_test::serial;
use vtuber_song_list::app::App;
use vtuber_song_list::models::song_diffs::{STATUS_APPROVED, STATUS_PENDING};

use super::session_prepare_data as pd;

#[tokio::test]
#[serial]
async fn member_can_create_diff_as_pending() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_video(&ctx, i64::from(channel.id)).await;
        let item = pd::create_song_item(&ctx, i64::from(video.id)).await;

        let user = pd::register_and_login(&request, "diffmember", "password123").await;
        let (key, val) = pd::auth_header(&user.token);

        let res = request
            .post(&format!("/api/member/song_items/{}/song_diffs", item.id))
            .add_header(key, val)
            .json(&serde_json::json!({
                "time": "00:02:00",
                "title": "New Title",
                "author": "New Artist",
            }))
            .await;

        assert_eq!(res.status_code(), 201);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(
            body["status"], STATUS_PENDING,
            "Member diff should be pending"
        );
        assert_eq!(body["title"], "New Title");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_can_create_diff_as_approved() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_video(&ctx, i64::from(channel.id)).await;
        let item = pd::create_song_item(&ctx, i64::from(video.id)).await;

        let admin = pd::register_and_login(&request, "diffadmin", "password123").await;
        pd::make_admin(&ctx, "diffadmin").await;
        let token = pd::login(&request, "diffadmin", "password123").await;
        let (key, val) = pd::auth_header(&token);

        let res = request
            .post(&format!("/api/member/song_items/{}/song_diffs", item.id))
            .add_header(key, val)
            .json(&serde_json::json!({
                "time": "00:03:00",
                "title": "Admin Title",
                "author": "Admin Artist",
            }))
            .await;

        assert_eq!(res.status_code(), 201);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(
            body["status"], STATUS_APPROVED,
            "Admin diff should be immediately approved"
        );

        // latest_diff_id on song_item should be updated
        let item_res = request.get(&format!("/api/song_items/{}", item.id)).await;
        assert_eq!(item_res.status_code(), 200);
        let item_body: serde_json::Value = serde_json::from_str(&item_res.text()).unwrap();
        assert_eq!(
            item_body["diff"]["title"], "Admin Title",
            "song_item should reflect admin's approved diff"
        );

        drop(admin); // suppress unused warning
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_create_diff_without_auth() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_video(&ctx, i64::from(channel.id)).await;
        let item = pd::create_song_item(&ctx, i64::from(video.id)).await;

        let res = request
            .post(&format!("/api/member/song_items/{}/song_diffs", item.id))
            .json(&serde_json::json!({ "title": "Unauthorized" }))
            .await;

        assert!(
            res.status_code() != 201,
            "Unauthenticated request should be rejected"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_diffs() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_video(&ctx, i64::from(channel.id)).await;
        let item = pd::create_song_item(&ctx, i64::from(video.id)).await;

        let user = pd::register_and_login(&request, "listdiffuser", "password123").await;
        let (key, val) = pd::auth_header(&user.token);

        // Create a diff
        request
            .post(&format!("/api/member/song_items/{}/song_diffs", item.id))
            .add_header(key.clone(), val.clone())
            .json(&serde_json::json!({ "title": "Listed Song" }))
            .await;

        let res = request
            .get(&format!("/api/member/song_items/{}/song_diffs", item.id))
            .add_header(key, val)
            .await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        let diffs = body.as_array().unwrap();
        assert!(
            !diffs.is_empty(),
            "Should return at least the initial approved diff plus the new pending diff"
        );
    })
    .await;
}
