use loco_rs::prelude::*;

use crate::workers::song_items_creator::{SongItemsCreatorWorker, SongItemsCreatorWorkerArgs};

pub struct CreateRecentVideosSongItems;

#[async_trait]
impl Task for CreateRecentVideosSongItems {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "create_recent_videos_song_items".to_string(),
            detail: "セトリ自動作成処理を実行する".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let worker = SongItemsCreatorWorker::build(app_context);
        worker.perform(SongItemsCreatorWorkerArgs {}).await?;
        Ok(())
    }
}
