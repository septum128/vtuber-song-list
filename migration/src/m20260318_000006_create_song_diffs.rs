use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "song_diffs",
            &[
                ("id", ColType::PkAuto),
                ("song_item_id", ColType::BigInteger),
                ("made_by_id", ColType::BigIntegerNull),
                ("comment_id", ColType::BigIntegerNull),
                ("time", ColType::StringNull),
                ("title", ColType::StringNull),
                ("author", ColType::StringNull),
                ("status", ColType::IntegerWithDefault(0)),
                ("kind", ColType::IntegerWithDefault(0)),
            ],
            &[
                ("song_items", "song_item_id"),
                ("users?", "made_by_id"),
                ("comments?", "comment_id"),
            ],
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_song_diffs_on_song_item_id")
                .table(Alias::new("song_diffs"))
                .col(Alias::new("song_item_id"))
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_song_diffs_on_made_by_id")
                .table(Alias::new("song_diffs"))
                .col(Alias::new("made_by_id"))
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_song_diffs_on_comment_id")
                .table(Alias::new("song_diffs"))
                .col(Alias::new("comment_id"))
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "song_diffs").await?;
        Ok(())
    }
}
