use loco_rs::testing::prelude::*;
use serial_test::serial;
use vtuber_song_list::app::App;

use super::session_prepare_data as pd;

#[tokio::test]
#[serial]
async fn get_one_returns_published_video() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_video(&ctx, channel.id).await;

        let res = request.get(&format!("/api/videos/{}", video.id)).await;
        assert_eq!(res.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["video_id"], "test_video_abc123");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn get_one_returns_404_for_unpublished_video() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_unpublished_video(&ctx, channel.id).await;

        let res = request.get(&format!("/api/videos/{}", video.id)).await;
        assert_eq!(
            res.status_code(),
            404,
            "Unpublished video should not be returned"
        );
    })
    .await;
}
