use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "comments",
            &[
                ("id", ColType::PkAuto),
                ("comment_id", ColType::StringUniq),
                ("video_id", ColType::BigInteger),
                ("author", ColType::String),
                ("content", ColType::String),
                ("response_json", ColType::Json),
                ("status", ColType::IntegerWithDefault(0)),
            ],
            &[("videos", "video_id")],
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_comments_on_video_id")
                .table(Alias::new("comments"))
                .col(Alias::new("video_id"))
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "comments").await?;
        Ok(())
    }
}
