use loco_rs::testing::prelude::*;
use serial_test::serial;
use vtuber_song_list::app::App;

use super::session_prepare_data as pd;

#[tokio::test]
#[serial]
async fn can_register() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .post("/api/user")
            .json(&serde_json::json!({
                "name": "testuser",
                "password": "password123",
                "password_confirmation": "password123",
            }))
            .await;

        assert_eq!(res.status_code(), 201, "Register should return 201");
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert!(body.get("token").is_some(), "Response should include token");
        assert_eq!(body["user"]["name"], "testuser");
        assert_eq!(
            body["user"]["kind"], 0,
            "New user should be kind=0 (member)"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_register_duplicate_name() {
    request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "name": "duplicate",
            "password": "password123",
            "password_confirmation": "password123",
        });
        request.post("/api/user").json(&payload).await;
        let res = request.post("/api/user").json(&payload).await;

        assert_eq!(res.status_code(), 400, "Duplicate name should return 400");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_register_password_mismatch() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .post("/api/user")
            .json(&serde_json::json!({
                "name": "mismatch",
                "password": "password123",
                "password_confirmation": "different",
            }))
            .await;

        assert_eq!(
            res.status_code(),
            400,
            "Password mismatch should return 400"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_login() {
    request::<App, _, _>(|request, _ctx| async move {
        pd::register_and_login(&request, "loginuser", "password123").await;

        let res = request
            .post("/api/session")
            .json(&serde_json::json!({
                "name": "loginuser",
                "password": "password123",
            }))
            .await;

        assert_eq!(res.status_code(), 200, "Login should return 200");
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert!(
            body.get("token").is_some(),
            "Login response should include token"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_login_wrong_password() {
    request::<App, _, _>(|request, _ctx| async move {
        pd::register_and_login(&request, "wrongpassuser", "password123").await;

        let res = request
            .post("/api/session")
            .json(&serde_json::json!({
                "name": "wrongpassuser",
                "password": "wrong",
            }))
            .await;

        assert_eq!(res.status_code(), 400, "Wrong password should return 400");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_login_unknown_user() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .post("/api/session")
            .json(&serde_json::json!({
                "name": "nonexistent",
                "password": "password123",
            }))
            .await;

        assert_eq!(res.status_code(), 400, "Unknown user should return 400");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_current_user() {
    request::<App, _, _>(|request, _ctx| async move {
        let user = pd::register_and_login(&request, "currentuser", "password123").await;
        let (key, val) = pd::auth_header(&user.token);

        let res = request.get("/api/user").add_header(key, val).await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(body["name"], "currentuser");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_get_current_user_without_token() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/user").await;
        assert!(
            res.status_code() != 200,
            "Should reject unauthenticated request"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_logout() {
    request::<App, _, _>(|request, _ctx| async move {
        let user = pd::register_and_login(&request, "logoutuser", "password123").await;
        let (key, val) = pd::auth_header(&user.token);

        let res = request.delete("/api/session").add_header(key, val).await;

        assert_eq!(res.status_code(), 200);
        let body: serde_json::Value = serde_json::from_str(&res.text()).unwrap();
        assert!(body.get("message").is_some());
    })
    .await;
}
