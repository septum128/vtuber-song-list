use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::{models::_entities::channels, workers::youtube_client::YouTubeClient};

pub struct FetchChannelIcons;

#[async_trait]
impl Task for FetchChannelIcons {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "fetch_channel_icons".to_string(),
            detail: "チャンネルアイコンURLを取得してDBに保存する".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let db = &app_context.db;
        let google_api_key =
            std::env::var("GOOGLE_API_KEY").map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
        let youtube_client = YouTubeClient::new(google_api_key);

        let channels = channels::Entity::find()
            .filter(channels::Column::IconUrl.is_null())
            .all(db)
            .await
            .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

        let total = channels.len();
        let mut updated = 0usize;

        for channel in channels {
            match youtube_client.fetch_channel_icon(&channel.channel_id).await {
                Ok(Some(url)) => {
                    let mut active: channels::ActiveModel = channel.into();
                    active.icon_url = ActiveValue::set(Some(url));
                    active
                        .update(db)
                        .await
                        .map_err(|e| loco_rs::Error::Any(Box::new(e)))?;
                    updated += 1;
                }
                Ok(None) => {
                    tracing::warn!("No icon found for channel {}", channel.channel_id);
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to fetch icon for channel {}: {e}",
                        channel.channel_id
                    );
                }
            }
        }

        tracing::info!("チャンネルアイコン取得完了: {total}件中{updated}件更新");
        Ok(())
    }
}
