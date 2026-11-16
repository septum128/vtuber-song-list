use loco_rs::testing::prelude::*;
use serial_test::serial;
use vtuber_song_list::app::App;

use super::session_prepare_data as pd;

#[tokio::test]
#[serial]
async fn get_one_returns_song_item_of_published_video() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_video(&ctx, channel.id).await;
        let item = pd::create_song_item(&ctx, video.id).await;

        let res = request.get(&format!("/api/song_items/{}", item.id)).await;
        assert_eq!(res.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["diff"]["title"], "Test Song");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn get_one_returns_404_for_song_item_of_unpublished_video() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;
        let video = pd::create_unpublished_video(&ctx, channel.id).await;
        let item = pd::create_song_item(&ctx, video.id).await;

        let res = request.get(&format!("/api/song_items/{}", item.id)).await;
        assert_eq!(
            res.status_code(),
            404,
            "Song item of an unpublished video should not be returned"
        );
    })
    .await;
}
