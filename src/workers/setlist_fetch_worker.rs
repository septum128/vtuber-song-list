use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use super::{
    openai_client::OpenAIClient,
    song_items_creator::{
        backfill_history_for_video, backfill_spotify_for_video, process_song_items_for_video,
    },
    spotify_client::SpotifyClient,
    youtube_client::YouTubeClient,
};
use crate::models::{
    _entities::{song_diffs as song_diffs_entity, song_items as song_items_entity, videos as videos_entity},
    videos::{self as videos_model, STATUS_FETCHED, STATUS_SONG_ITEMS_CREATED},
};

pub struct SetlistFetchWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SetlistFetchWorkerArgs {
    pub video_ids: Vec<i32>,
    pub force: bool,
}

#[async_trait]
impl BackgroundWorker<SetlistFetchWorkerArgs> for SetlistFetchWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: SetlistFetchWorkerArgs) -> Result<()> {
        let db = &self.ctx.db;

        let google_api_key =
            std::env::var("GOOGLE_API_KEY").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let openai_api_key =
            std::env::var("OPENAI_API_KEY").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let spotify_client_id =
            std::env::var("SPOTIFY_CLIENT_ID").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let spotify_client_secret = std::env::var("SPOTIFY_CLIENT_SECRET")
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

        let youtube_client = YouTubeClient::new(google_api_key);
        let openai_client = OpenAIClient::new(openai_api_key);
        let spotify_client = SpotifyClient::new(spotify_client_id, spotify_client_secret);

        let mut total_created = 0usize;

        for video_id in &args.video_ids {
            let Some(video) = videos_entity::Entity::find_by_id(*video_id)
                .one(db)
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?
            else {
                tracing::warn!("SetlistFetchWorker: video {video_id} not found, skipping");
                continue;
            };

            if args.force {
                reset_video_setlist(db, &video).await?;
            } else if video.status >= STATUS_SONG_ITEMS_CREATED {
                tracing::info!(
                    "SetlistFetchWorker: video {video_id} already processed (status={}), skipping",
                    video.status
                );
                continue;
            }

            // Re-fetch after potential status reset
            let Some(video) = videos_entity::Entity::find_by_id(*video_id)
                .one(db)
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?
            else {
                continue;
            };

            let created =
                process_song_items_for_video(db, &video, &youtube_client, &openai_client).await?;
            total_created += created;

            // Run backfill only if song items were created
            let Some(video) = videos_entity::Entity::find_by_id(*video_id)
                .one(db)
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?
            else {
                continue;
            };

            if video.status >= STATUS_SONG_ITEMS_CREATED {
                backfill_history_for_video(db, &video).await?;

                let Some(video) = videos_entity::Entity::find_by_id(*video_id)
                    .one(db)
                    .await
                    .map_err(|e| loco_rs::Error::Any(Box::new(e)))?
                else {
                    continue;
                };

                backfill_spotify_for_video(db, &video, &spotify_client).await?;
            }
        }

        tracing::info!("SetlistFetchWorker完了: {}件の曲を作成", total_created);
        Ok(())
    }
}

/// Deletes all song items and diffs for `video`, then resets its status to `STATUS_FETCHED`.
async fn reset_video_setlist(
    db: &sea_orm::DatabaseConnection,
    video: &videos_entity::Model,
) -> Result<()> {
    let video_id = i64::from(video.id);

    let items = song_items_entity::Entity::find()
        .filter(song_items_entity::Column::VideoId.eq(video_id))
        .all(db)
        .await
        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

    if !items.is_empty() {
        let item_ids: Vec<i64> = items.iter().map(|i| i64::from(i.id)).collect();

        // Null latest_diff_id to avoid FK constraint violation when deleting song_diffs
        for item in items {
            let mut active: song_items_entity::ActiveModel = item.into();
            active.latest_diff_id = ActiveValue::set(None);
            active
                .update(db)
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        }

        song_diffs_entity::Entity::delete_many()
            .filter(song_diffs_entity::Column::SongItemId.is_in(item_ids))
            .exec(db)
            .await
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

        song_items_entity::Entity::delete_many()
            .filter(song_items_entity::Column::VideoId.eq(video_id))
            .exec(db)
            .await
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
    }

    videos_model::Model::update_status(db, video.id, STATUS_FETCHED)
        .await
        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

    Ok(())
}
