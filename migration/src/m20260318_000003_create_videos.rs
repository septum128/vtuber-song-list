use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "videos",
            &[
                ("id", ColType::PkAuto),
                ("channel_id", ColType::BigInteger),
                ("video_id", ColType::StringUniq),
                ("title", ColType::String),
                ("response_json", ColType::Json),
                ("kind", ColType::IntegerWithDefault(0)),
                ("status", ColType::IntegerWithDefault(0)),
                ("published", ColType::BooleanWithDefault(true)),
                ("published_at", ColType::TimestampWithTimeZone),
            ],
            &[("channels", "channel_id")],
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_videos_on_channel_id")
                .table(Alias::new("videos"))
                .col(Alias::new("channel_id"))
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "videos").await?;
        Ok(())
    }
}
