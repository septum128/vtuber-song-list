use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // The Spotify backfill used to store the video statuses 30 and 35 in
        // `song_diffs.status`. Only a song item's latest diff was backfilled, and a
        // diff becomes the latest only once approved, so these rows were approved (10).
        m.get_connection()
            .execute_unprepared(
                r#"UPDATE "song_diffs" SET "status" = 10 WHERE "status" IN (30, 35)"#,
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _m: &SchemaManager) -> Result<(), DbErr> {
        // The original 30/35 values carried no approval meaning, so there is nothing to restore.
        Ok(())
    }
}
