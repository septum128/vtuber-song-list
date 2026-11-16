use loco_rs::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use super::{
    openai_client::OpenAIClient,
    song_items_creator::{
        backfill_history_for_video, backfill_spotify_for_video, process_song_items_for_video,
    },
    spotify_client::SpotifyClient,
    youtube_client::YouTubeClient,
};
use crate::models::{_entities::videos as videos_entity, videos::STATUS_SONG_ITEMS_CREATED};

pub struct SetlistFetchWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SetlistFetchWorkerArgs {
    pub video_ids: Vec<i64>,
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
        let spotify_client_secret =
            std::env::var("SPOTIFY_CLIENT_SECRET").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

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

            if !args.force && video.status >= STATUS_SONG_ITEMS_CREATED {
                tracing::info!(
                    "SetlistFetchWorker: video {video_id} already processed (status={}), skipping",
                    video.status
                );
                continue;
            }

            // With force, the existing setlist is replaced only after new entries are extracted.
            let created = process_song_items_for_video(
                db,
                &video,
                &youtube_client,
                &openai_client,
                args.force,
            )
            .await?;
            total_created += created;

            // Run backfill only if song items were created
            if created == 0 {
                continue;
            }

            let Some(video) = videos_entity::Entity::find_by_id(*video_id)
                .one(db)
                .await
                .map_err(|e| loco_rs::Error::Any(Box::new(e)))?
            else {
                continue;
            };

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

        tracing::info!("SetlistFetchWorker完了: {}件の曲を作成", total_created);
        Ok(())
    }
}
