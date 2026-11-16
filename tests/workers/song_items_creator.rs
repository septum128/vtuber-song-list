use loco_rs::{app::AppContext, testing::prelude::*};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serial_test::serial;
use vtuber_song_list::{
    app::App,
    models::{
        _entities::{channels, comments, song_diffs, song_items, videos},
        comments::{ActiveModel as CommentActive, UpsertCommentParams, STATUS_COMPLETED},
        song_items::{ActiveModel as SongItemActive, CreateSongItemParams},
        videos::{
            STATUS_COMPLETED as VIDEO_STATUS_COMPLETED, STATUS_FETCHED, STATUS_SONG_ITEMS_CREATED,
        },
    },
    workers::song_items_creator::{save_song_entries, PendingSongEntry},
};

const MISSING_COMMENT_ID: i64 = 999_999;

async fn create_video(ctx: &AppContext, status: i32) -> videos::Model {
    let channel = channels::ActiveModel {
        channel_id: ActiveValue::set("UC_worker_test".to_string()),
        custom_name: ActiveValue::set("Worker Test".to_string()),
        kind: ActiveValue::set(100),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    videos::ActiveModel {
        channel_id: ActiveValue::set(channel.id),
        video_id: ActiveValue::set("worker_test_video".to_string()),
        title: ActiveValue::set("歌枠".to_string()),
        response_json: ActiveValue::set(serde_json::json!({})),
        kind: ActiveValue::set(10),
        status: ActiveValue::set(status),
        published: ActiveValue::set(true),
        published_at: ActiveValue::set(chrono::Utc::now().fixed_offset()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap()
}

async fn create_comment(ctx: &AppContext, video_id: i64) -> comments::Model {
    CommentActive::upsert(
        &ctx.db,
        UpsertCommentParams {
            comment_id: "worker_test_comment".to_string(),
            video_id,
            author: "viewer".to_string(),
            content: "00:01:00 Song\n00:05:00 Song\n00:09:00 Song".to_string(),
            response_json: serde_json::json!({}),
        },
    )
    .await
    .unwrap()
}

async fn create_existing_item(ctx: &AppContext, video_id: i64) {
    SongItemActive::create_with_diff(
        &ctx.db,
        CreateSongItemParams {
            video_id,
            time: Some("00:01:00".to_string()),
            title: Some("Old Song".to_string()),
            author: Some("Old Artist".to_string()),
        },
    )
    .await
    .unwrap();
}

fn entry(comment_id: i64, time: &str, title: &str, author: &str) -> PendingSongEntry {
    PendingSongEntry {
        comment_id,
        time: time.to_string(),
        title: title.to_string(),
        author: author.to_string(),
    }
}

/// Returns the latest diff titles of the video's song items, sorted.
async fn item_titles(ctx: &AppContext, video_id: i64) -> Vec<String> {
    let items = song_items::Entity::find()
        .filter(song_items::Column::VideoId.eq(video_id))
        .all(&ctx.db)
        .await
        .unwrap();
    let mut titles = Vec::new();
    for item in items {
        let diff = song_diffs::Entity::find_by_id(item.latest_diff_id.unwrap())
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
        titles.push(diff.title.unwrap());
    }
    titles.sort();
    titles
}

async fn video_status(ctx: &AppContext, id: i64) -> i32 {
    videos::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap()
        .status
}

#[tokio::test]
#[serial]
async fn saves_entries_and_updates_status() {
    let boot = boot_test::<App>().await.unwrap();
    let ctx = &boot.app_context;
    let video = create_video(ctx, STATUS_FETCHED).await;
    let comment = create_comment(ctx, video.id).await;

    let created = save_song_entries(
        &ctx.db,
        video.id,
        vec![
            entry(comment.id, "00:01:00", "Song A", "Artist A"),
            entry(comment.id, "00:05:00", "Song B", "Artist B"),
        ],
        &[comment.id],
        false,
    )
    .await
    .unwrap();

    assert_eq!(created, 2);
    assert_eq!(item_titles(ctx, video.id).await, vec!["Song A", "Song B"]);
    assert_eq!(video_status(ctx, video.id).await, STATUS_SONG_ITEMS_CREATED);
    let comment = comments::Entity::find_by_id(comment.id)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(comment.status, STATUS_COMPLETED);
}

#[tokio::test]
#[serial]
async fn rolls_back_all_items_when_save_fails_midway() {
    let boot = boot_test::<App>().await.unwrap();
    let ctx = &boot.app_context;
    let video = create_video(ctx, STATUS_FETCHED).await;
    let comment = create_comment(ctx, video.id).await;

    // The second entry references a missing comment, so its diff violates the FK constraint.
    let result = save_song_entries(
        &ctx.db,
        video.id,
        vec![
            entry(comment.id, "00:01:00", "Song A", "Artist A"),
            entry(MISSING_COMMENT_ID, "00:05:00", "Song B", "Artist B"),
        ],
        &[comment.id],
        false,
    )
    .await;

    assert!(result.is_err());
    assert!(
        item_titles(ctx, video.id).await.is_empty(),
        "No song items should remain after a failed save"
    );
    assert_eq!(video_status(ctx, video.id).await, STATUS_FETCHED);
}

#[tokio::test]
#[serial]
async fn replaces_existing_setlist() {
    let boot = boot_test::<App>().await.unwrap();
    let ctx = &boot.app_context;
    let video = create_video(ctx, VIDEO_STATUS_COMPLETED).await;
    create_existing_item(ctx, video.id).await;
    let comment = create_comment(ctx, video.id).await;

    let created = save_song_entries(
        &ctx.db,
        video.id,
        vec![entry(comment.id, "00:01:00", "New Song", "New Artist")],
        &[comment.id],
        true,
    )
    .await
    .unwrap();

    assert_eq!(created, 1);
    assert_eq!(item_titles(ctx, video.id).await, vec!["New Song"]);
    assert_eq!(video_status(ctx, video.id).await, STATUS_SONG_ITEMS_CREATED);
}

#[tokio::test]
#[serial]
async fn keeps_existing_setlist_when_nothing_extracted() {
    let boot = boot_test::<App>().await.unwrap();
    let ctx = &boot.app_context;
    let video = create_video(ctx, VIDEO_STATUS_COMPLETED).await;
    create_existing_item(ctx, video.id).await;

    let created = save_song_entries(&ctx.db, video.id, vec![], &[], true)
        .await
        .unwrap();

    assert_eq!(created, 0);
    assert_eq!(item_titles(ctx, video.id).await, vec!["Old Song"]);
    assert_eq!(video_status(ctx, video.id).await, VIDEO_STATUS_COMPLETED);
}

#[tokio::test]
#[serial]
async fn keeps_existing_setlist_when_replace_fails() {
    let boot = boot_test::<App>().await.unwrap();
    let ctx = &boot.app_context;
    let video = create_video(ctx, VIDEO_STATUS_COMPLETED).await;
    create_existing_item(ctx, video.id).await;
    let comment = create_comment(ctx, video.id).await;

    let result = save_song_entries(
        &ctx.db,
        video.id,
        vec![
            entry(comment.id, "00:01:00", "New Song", "New Artist"),
            entry(MISSING_COMMENT_ID, "00:05:00", "Broken", "Broken"),
        ],
        &[comment.id],
        true,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        item_titles(ctx, video.id).await,
        vec!["Old Song"],
        "Existing setlist should survive a failed replace"
    );
    assert_eq!(video_status(ctx, video.id).await, VIDEO_STATUS_COMPLETED);
}

#[tokio::test]
#[serial]
async fn saves_entries_with_empty_author_for_backfill() {
    let boot = boot_test::<App>().await.unwrap();
    let ctx = &boot.app_context;
    let video = create_video(ctx, STATUS_FETCHED).await;
    let comment = create_comment(ctx, video.id).await;

    let created = save_song_entries(
        &ctx.db,
        video.id,
        vec![entry(comment.id, "00:01:00", "Unknown Artist Song", "")],
        &[comment.id],
        false,
    )
    .await
    .unwrap();

    assert_eq!(created, 1);
    let diff = song_diffs::Entity::find()
        .filter(song_diffs::Column::Title.eq("Unknown Artist Song"))
        .one(&ctx.db)
        .await
        .unwrap()
        .expect("Entry with an empty author should be saved");
    assert_eq!(diff.author.as_deref(), Some(""));
}
