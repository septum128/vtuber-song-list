use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "song_items",
            &[
                ("id", ColType::PkAuto),
                ("video_id", ColType::BigInteger),
                ("latest_diff_id", ColType::IntegerNull),
            ],
            &[("videos", "video_id")],
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_song_items_on_video_id")
                .table(Alias::new("song_items"))
                .col(Alias::new("video_id"))
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_song_items_on_latest_diff_id")
                .table(Alias::new("song_items"))
                .col(Alias::new("latest_diff_id"))
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "song_items").await?;
        Ok(())
    }
}
