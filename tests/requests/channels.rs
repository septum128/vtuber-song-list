use loco_rs::testing::prelude::*;
use serial_test::serial;
use vtuber_song_list::app::App;

use super::session_prepare_data as pd;

#[tokio::test]
#[serial]
async fn list_returns_only_published() {
    request::<App, _, _>(|_request, ctx| async move {
        let _published = pd::create_channel(&ctx).await;
        let _hidden = pd::create_hidden_channel(&ctx).await;

        let res = _request.get("/api/channels").await;
        assert_eq!(res.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        let channels = body.as_array().unwrap();

        assert!(
            channels.iter().all(|c| c["kind"] == 100),
            "Only published channels (kind=100) should be returned"
        );
        assert!(
            !channels
                .iter()
                .any(|c| c["channel_id"] == "UC_hidden_channel_001"),
            "Hidden channel should not appear"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn get_one_returns_channel() {
    request::<App, _, _>(|request, ctx| async move {
        let channel = pd::create_channel(&ctx).await;

        let res = request.get(&format!("/api/channels/{}", channel.id)).await;
        assert_eq!(res.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["channel_id"], "UC_test_channel_001");
        assert_eq!(body["custom_name"], "Test Channel");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn get_one_returns_404_for_unknown_id() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/channels/99999").await;
        assert_eq!(res.status_code(), 404, "Unknown channel should return 404");
    })
    .await;
}
