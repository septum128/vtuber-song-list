use loco_rs::testing::prelude::*;
use serial_test::serial;
use vtuber_song_list::app::App;
use vtuber_song_list::models::song_diffs::{STATUS_APPROVED, STATUS_REJECTED};

use crate::requests::session_prepare_data as pd;

async fn setup_pending_diff(
    request: &loco_rs::TestServer,
    ctx: &loco_rs::app::AppContext,
) -> (
    vtuber_song_list::models::_entities::song_items::Model,
    i64, // diff id
) {
    let channel = pd::create_channel(ctx).await;
    let video = pd::create_video(ctx, i64::from(channel.id)).await;
    let item = pd::create_song_item(ctx, i64::from(video.id)).await;

    // Submit a pending diff as a regular member
    let member = pd::register_and_login(request, "pending_submitter", "password123").await;
    let (key, val) = pd::auth_header(&member.token);
    let res = request
        .post(&format!("/api/member/song_items/{}/song_diffs", item.id))
        .add_header(key, val)
        .json(&serde_json::json!({
            "time": "00:05:00",
            "title": "Pending Song",
            "author": "Pending Artist",
        }))
        .await;
    let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
    let diff_id = body["id"].as_i64().unwrap();

    (item, diff_id)
}

#[tokio::test]
#[serial]
async fn admin_can_approve_diff() {
    request::<App, _, _>(|request, ctx| async move {
        let (item, diff_id) = setup_pending_diff(&request, &ctx).await;

        let admin = pd::register_and_login(&request, "approver_admin", "password123").await;
        pd::make_admin(&ctx, "approver_admin").await;
        let token = pd::login(&request, "approver_admin", "password123").await;
        let (key, val) = pd::auth_header(&token);

        let res = request
            .patch(&format!("/api/admin/song_diffs/{diff_id}/approve"))
            .add_header(key, val)
            .await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["status"], STATUS_APPROVED, "Diff should be approved");

        // Verify latest_diff_id on song_item updated
        let item_res = request.get(&format!("/api/song_items/{}", item.id)).await;
        let item_body: serde_json::Value = serde_json::from_str(&item_res.text()).unwrap();
        assert_eq!(
            item_body["diff"]["title"], "Pending Song",
            "song_item should now reflect approved diff"
        );

        drop(admin);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_can_reject_diff() {
    request::<App, _, _>(|request, ctx| async move {
        let (_item, diff_id) = setup_pending_diff(&request, &ctx).await;

        let admin = pd::register_and_login(&request, "rejecter_admin", "password123").await;
        pd::make_admin(&ctx, "rejecter_admin").await;
        let token = pd::login(&request, "rejecter_admin", "password123").await;
        let (key, val) = pd::auth_header(&token);

        let res = request
            .patch(&format!("/api/admin/song_diffs/{diff_id}/reject"))
            .add_header(key, val)
            .await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["status"], STATUS_REJECTED, "Diff should be rejected");

        drop(admin);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn member_cannot_approve_diff() {
    request::<App, _, _>(|request, ctx| async move {
        let (_item, diff_id) = setup_pending_diff(&request, &ctx).await;

        // Regular member tries to approve
        let member = pd::register_and_login(&request, "not_admin_member", "password123").await;
        let (key, val) = pd::auth_header(&member.token);

        let res = request
            .patch(&format!("/api/admin/song_diffs/{diff_id}/approve"))
            .add_header(key, val)
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
async fn admin_can_list_pending_diffs() {
    request::<App, _, _>(|request, ctx| async move {
        let (_item, _diff_id) = setup_pending_diff(&request, &ctx).await;

        let admin = pd::register_and_login(&request, "list_admin", "password123").await;
        pd::make_admin(&ctx, "list_admin").await;
        let token = pd::login(&request, "list_admin", "password123").await;
        let (key, val) = pd::auth_header(&token);

        let res = request
            .get("/api/admin/song_diffs?status=0")
            .add_header(key, val)
            .await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        let diffs = body.as_array().unwrap();
        assert!(!diffs.is_empty(), "Should return at least one pending diff");
        assert!(
            diffs.iter().all(|d| d["status"] == 0),
            "All returned diffs should be pending (status=0)"
        );

        drop(admin);
    })
    .await;
}
