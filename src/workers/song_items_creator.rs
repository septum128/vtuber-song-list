use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbBackend, FromQueryResult, QueryFilter, Statement,
};
use serde::{Deserialize, Serialize};

use super::{
    openai_client::OpenAIClient, slack_client::SlackClient, spotify_client::SpotifyClient,
    youtube_client::YouTubeClient,
};
use crate::models::{
    _entities::{song_diffs as song_diffs_entity, song_items, videos as videos_entity},
    channels,
    comments::{self as comments_model, UpsertCommentParams},
    song_diffs as song_diffs_model,
    videos::{
        self as videos_model, STATUS_COMMENTS_DISABLED, STATUS_COMPLETED, STATUS_FETCHED,
        STATUS_FETCHED_HISTORY, STATUS_SONG_ITEMS_CREATED, STATUS_SPOTIFY_COMPLETED,
        STATUS_SPOTIFY_FETCHED,
    },
};

pub struct SongItemsCreatorWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SongItemsCreatorWorkerArgs {}

const SONG_LIVE_KEYWORDS: &[&str] = &[
    "歌枠",
    "うたわく",
    "歌ってみた",
    "弾き語り",
    "歌配信",
    "カラオケ",
    "karaoke",
    "singing",
    "song",
];

#[derive(Debug, FromQueryResult)]
struct DiffWithTitle {
    id: i32,
    title: Option<String>,
    #[allow(dead_code)]
    song_item_id: i64,
}

fn is_song_live(title: &str) -> bool {
    let lower = title.to_lowercase();
    SONG_LIVE_KEYWORDS
        .iter()
        .any(|kw| lower.contains(&kw.to_lowercase()))
}

#[async_trait]
impl BackgroundWorker<SongItemsCreatorWorkerArgs> for SongItemsCreatorWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, _args: SongItemsCreatorWorkerArgs) -> Result<()> {
        let db = &self.ctx.db;

        let google_api_key =
            std::env::var("GOOGLE_API_KEY").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let openai_api_key =
            std::env::var("OPENAI_API_KEY").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let spotify_client_id =
            std::env::var("SPOTIFY_CLIENT_ID").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let spotify_client_secret =
            std::env::var("SPOTIFY_CLIENT_SECRET").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let slack_webhook_url = std::env::var("SLACK_WEBHOOK_URL").ok();

        let slack_client = slack_webhook_url
            .as_deref()
            .map(|url| SlackClient::new(url.to_string()));

        let result = self
            .run_pipeline(
                db,
                &google_api_key,
                &openai_api_key,
                &spotify_client_id,
                &spotify_client_secret,
            )
            .await;

        match result {
            Ok((processed_videos, created_songs)) => {
                let msg = format!(
                    "セトリ自動作成完了: {processed_videos}件処理, {created_songs}件の曲を作成"
                );
                tracing::info!("{}", msg);
                if let Some(slack) = &slack_client {
                    if let Err(e) = slack.notify(&msg).await {
                        tracing::warn!("Slack notification failed: {e}");
                    }
                }
            }
            Err(ref e) => {
                let msg = format!("セトリ自動作成エラー: {e}");
                tracing::error!("{}", msg);
                if let Some(slack) = &slack_client {
                    if let Err(slack_err) = slack.notify(&msg).await {
                        tracing::warn!("Slack notification failed: {slack_err}");
                    }
                }
                return result.map(|_| ());
            }
        }

        Ok(())
    }
}

impl SongItemsCreatorWorker {
    async fn run_pipeline(
        &self,
        db: &sea_orm::DatabaseConnection,
        google_api_key: &str,
        openai_api_key: &str,
        spotify_client_id: &str,
        spotify_client_secret: &str,
    ) -> Result<(usize, usize)> {
        let youtube_client = YouTubeClient::new(google_api_key.to_string());
        let openai_client = OpenAIClient::new(openai_api_key.to_string());
        let spotify_client = SpotifyClient::new(
            spotify_client_id.to_string(),
            spotify_client_secret.to_string(),
        );

        // Step 2: Fetch RSS and insert new videos
        self.fetch_and_insert_videos(db, &youtube_client).await?;

        // Step 3: Process incomplete videos (fetch comments + create song items)
        let mut processed_videos = 0usize;
        let mut created_songs = 0usize;
        let (p, c) = self
            .process_song_items(db, &youtube_client, &openai_client)
            .await?;
        processed_videos += p;
        created_songs += c;

        // Step 4: History-based author backfill
        self.backfill_author_from_history(db).await?;

        // Step 5: Spotify author backfill
        self.backfill_author_from_spotify(db, &spotify_client)
            .await?;

        Ok((processed_videos, created_songs))
    }

    /// Step 2: fetch RSS for all channels and insert new videos.
    /// For live broadcasts, `published_at` is set to `actualStartTime` from the `YouTube` API.
    /// Regular videos fall back to the RSS `<published>` date.
    async fn fetch_and_insert_videos(
        &self,
        db: &sea_orm::DatabaseConnection,
        youtube_client: &YouTubeClient,
    ) -> Result<()> {
        let channels = channels::Model::find_all_for_rss(db)
            .await
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

        for channel in channels {
            let entries = match youtube_client.fetch_rss_entries(&channel.channel_id).await {
                Ok(e) => e,
                Err(e) => {
                    tracing::warn!(
                        "Failed to fetch RSS for channel {}: {e}",
                        channel.channel_id
                    );
                    continue;
                }
            };

            // Collect only new video IDs (not yet in DB), preserving published date from RSS.
            let mut new_entries = Vec::new();
            for entry in entries {
                let existing = videos_entity::Entity::find()
                    .filter(videos_entity::Column::VideoId.eq(&entry.video_id))
                    .one(db)
                    .await
                    .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

                if existing.is_none() {
                    new_entries.push(entry);
                }
            }

            if new_entries.is_empty() {
                continue;
            }

            // Fetch video details from YouTube API to get actualStartTime for live broadcasts.
            let video_ids: Vec<&str> = new_entries.iter().map(|e| e.video_id.as_str()).collect();
            let video_infos = match youtube_client.fetch_video_info(&video_ids).await {
                Ok(infos) => infos,
                Err(e) => {
                    tracing::warn!(
                        "Failed to fetch video info for channel {}: {e}",
                        channel.channel_id
                    );
                    continue;
                }
            };

            // Build a map from video_id to actualStartTime.
            let start_time_map: std::collections::HashMap<String, Option<String>> = video_infos
                .into_iter()
                .map(|info| (info.video_id, info.actual_start_time))
                .collect();

            for entry in new_entries {
                // Use actualStartTime for live broadcasts; fall back to RSS published date.
                let published_at_str = start_time_map
                    .get(&entry.video_id)
                    .and_then(Option::as_deref)
                    .unwrap_or(&entry.published);
                let published_at = parse_published_at(published_at_str);

                let new_video = videos_entity::ActiveModel {
                    video_id: ActiveValue::set(entry.video_id.clone()),
                    channel_id: ActiveValue::set(i64::from(channel.id)),
                    title: ActiveValue::set(entry.title),
                    response_json: ActiveValue::set(serde_json::json!({})),
                    kind: ActiveValue::set(0),
                    status: ActiveValue::set(STATUS_FETCHED),
                    published: ActiveValue::set(false),
                    published_at: ActiveValue::set(published_at),
                    ..Default::default()
                };

                if let Err(e) = new_video.insert(db).await {
                    tracing::warn!("Failed to insert video {}: {e}", entry.video_id);
                }
            }
        }

        Ok(())
    }

    /// Step 3: For each incomplete song-live video, fetch comments and create song items.
    #[allow(clippy::too_many_lines)]
    async fn process_song_items(
        &self,
        db: &sea_orm::DatabaseConnection,
        youtube_client: &YouTubeClient,
        openai_client: &OpenAIClient,
    ) -> Result<(usize, usize)> {
        let videos = videos_model::Model::find_incomplete(db)
            .await
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

        let mut processed = 0usize;
        let mut created = 0usize;

        for video in videos {
            if !is_song_live(&video.title) {
                continue;
            }

            let comments = match youtube_client.fetch_comments(&video.video_id).await {
                Ok(c) => c,
                Err(e) => {
                    if e.status() == Some(reqwest::StatusCode::FORBIDDEN) {
                        tracing::warn!(
                            "Comments disabled for video {}, marking as skipped",
                            video.video_id
                        );
                        videos_model::Model::update_status(db, video.id, STATUS_COMMENTS_DISABLED)
                            .await
                            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
                    } else {
                        tracing::warn!(
                            "Failed to fetch comments for video {}: {e}",
                            video.video_id
                        );
                    }
                    continue;
                }
            };

            // Upsert all comments
            let mut saved_comments = Vec::new();
            for comment_info in comments {
                let saved = comments_model::ActiveModel::upsert(
                    db,
                    UpsertCommentParams {
                        comment_id: comment_info.comment_id,
                        video_id: i64::from(video.id),
                        author: comment_info.author,
                        content: comment_info.content,
                        response_json: comment_info.response_json,
                    },
                )
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
                saved_comments.push(saved);
            }

            // Process setlist comments with OpenAI
            let mut video_created = 0usize;
            for comment in &saved_comments {
                if !comment.is_setlist() {
                    continue;
                }

                let entries = match openai_client.extract_setlist(&comment.content).await {
                    Ok(e) => e,
                    Err(e) => {
                        tracing::warn!("OpenAI failed for comment {}: {e}", comment.id);
                        continue;
                    }
                };

                for entry in entries {
                    // Create song_item
                    let item = song_items::ActiveModel {
                        video_id: ActiveValue::set(i64::from(video.id)),
                        latest_diff_id: ActiveValue::set(None),
                        ..Default::default()
                    }
                    .insert(db)
                    .await
                    .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

                    // Create song_diff (auto, approved) and update latest_diff_id
                    let time = if entry.time.is_empty() {
                        None
                    } else {
                        Some(entry.time)
                    };
                    let title = if entry.title.is_empty() {
                        None
                    } else {
                        Some(entry.title)
                    };
                    let author = if entry.author.is_empty() {
                        None
                    } else {
                        Some(entry.author)
                    };

                    song_diffs_model::ActiveModel::create_auto(
                        db,
                        i64::from(item.id),
                        Some(i64::from(comment.id)),
                        time,
                        title,
                        author,
                    )
                    .await
                    .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

                    video_created += 1;
                }

                // Mark comment completed
                comments_model::Model::mark_completed(db, comment.id)
                    .await
                    .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
            }

            if video_created > 0 {
                videos_model::Model::update_status(db, video.id, STATUS_SONG_ITEMS_CREATED)
                    .await
                    .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
                processed += 1;
                created += video_created;
            }
        }

        Ok((processed, created))
    }

    /// Step 4: Fill empty `author` fields from historical approved diffs with the same title.
    async fn backfill_author_from_history(&self, db: &sea_orm::DatabaseConnection) -> Result<()> {
        let videos = videos_entity::Entity::find()
            .filter(videos_entity::Column::Status.eq(STATUS_SONG_ITEMS_CREATED))
            .all(db)
            .await
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

        for video in videos {
            let empty_diffs = find_empty_author_diffs(db, i64::from(video.id)).await?;

            for diff_row in empty_diffs {
                let Some(title) = &diff_row.title else {
                    continue;
                };
                if title.is_empty() {
                    continue;
                }
                if let Some(author) = find_historical_author(db, title).await? {
                    update_diff_author(db, diff_row.id, &author).await?;
                }
            }

            videos_model::Model::update_status(db, video.id, STATUS_FETCHED_HISTORY)
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        }

        Ok(())
    }

    /// Step 5: Fill empty `author` fields using Spotify search.
    async fn backfill_author_from_spotify(
        &self,
        db: &sea_orm::DatabaseConnection,
        spotify_client: &SpotifyClient,
    ) -> Result<()> {
        let videos = videos_entity::Entity::find()
            .filter(videos_entity::Column::Status.eq(STATUS_FETCHED_HISTORY))
            .all(db)
            .await
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

        for video in videos {
            let empty_diffs = find_empty_author_diffs(db, i64::from(video.id)).await?;

            for diff_row in empty_diffs {
                let Some(title) = &diff_row.title else {
                    continue;
                };
                if title.is_empty() {
                    continue;
                }

                match spotify_client.search_artist(title).await {
                    Ok(Some(author)) => {
                        update_diff_author(db, diff_row.id, &author).await?;
                        update_diff_status(db, diff_row.id, STATUS_SPOTIFY_COMPLETED).await?;
                    }
                    Ok(None) => {
                        update_diff_status(db, diff_row.id, STATUS_SPOTIFY_FETCHED).await?;
                    }
                    Err(e) => {
                        tracing::warn!("Spotify search failed for '{}': {e}", title);
                    }
                }
            }

            videos_model::Model::update_status(db, video.id, STATUS_COMPLETED)
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        }

        Ok(())
    }
}

/// Finds `song_diffs` with empty author linked to the video via `song_items.latest_diff_id`.
async fn find_empty_author_diffs(
    db: &sea_orm::DatabaseConnection,
    video_id: i64,
) -> Result<Vec<DiffWithTitle>> {
    let sql = r"
        SELECT sd.id, sd.title, sd.song_item_id
        FROM song_diffs sd
        INNER JOIN song_items si ON sd.id = si.latest_diff_id
        WHERE si.video_id = $1
          AND (sd.author IS NULL OR sd.author = '')
    ";
    DiffWithTitle::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        [video_id.into()],
    ))
    .all(db)
    .await
    .map_err(|e| loco_rs::Error::Any(Box::new(e)))
}

/// Finds the most recent approved author for a given song title.
async fn find_historical_author(
    db: &sea_orm::DatabaseConnection,
    title: &str,
) -> Result<Option<String>> {
    #[derive(FromQueryResult)]
    struct AuthorRow {
        author: String,
    }

    let sql = r"
        SELECT author FROM song_diffs
        WHERE title = $1
          AND status = 10
          AND (author IS NOT NULL AND author != '')
        ORDER BY created_at DESC
        LIMIT 1
    ";
    let row = AuthorRow::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        [title.into()],
    ))
    .one(db)
    .await
    .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

    Ok(row.map(|r| r.author))
}

/// Updates the `author` field of a song diff.
async fn update_diff_author(
    db: &sea_orm::DatabaseConnection,
    diff_id: i32,
    author: &str,
) -> Result<()> {
    let Some(diff) = song_diffs_entity::Entity::find_by_id(diff_id)
        .one(db)
        .await
        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?
    else {
        return Ok(());
    };
    let mut active: song_diffs_entity::ActiveModel = diff.into();
    active.author = ActiveValue::set(Some(author.to_string()));
    active
        .update(db)
        .await
        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
    Ok(())
}

/// Updates the `status` field of a song diff.
async fn update_diff_status(
    db: &sea_orm::DatabaseConnection,
    diff_id: i32,
    status: i32,
) -> Result<()> {
    let Some(diff) = song_diffs_entity::Entity::find_by_id(diff_id)
        .one(db)
        .await
        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?
    else {
        return Ok(());
    };
    let mut active: song_diffs_entity::ActiveModel = diff.into();
    active.status = ActiveValue::set(status);
    active
        .update(db)
        .await
        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
    Ok(())
}

fn parse_published_at(s: &str) -> sea_orm::prelude::DateTimeWithTimeZone {
    use chrono::{DateTime, Utc};
    DateTime::parse_from_rfc3339(s).unwrap_or_else(|_| Utc::now().fixed_offset())
}
