use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "channels",
            &[
                ("id", ColType::PkAuto),
                ("channel_id", ColType::StringUniq),
                ("name", ColType::StringNull),
                ("custom_name", ColType::StringWithDefault(String::new())),
                ("twitter_id", ColType::StringNull),
                ("response_json", ColType::JsonNull),
                ("kind", ColType::IntegerWithDefault(0)),
                ("status", ColType::IntegerWithDefault(0)),
            ],
            &[],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "channels").await?;
        Ok(())
    }
}
