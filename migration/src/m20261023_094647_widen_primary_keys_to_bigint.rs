use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Tables created with `ColType::PkAuto`.
///
/// Loco 0.16 created that column as `integer`, Loco 1.x creates it as `bigint`,
/// and sqlx 0.9 refuses to decode `INT4` into the `i64` used by the entities.
/// Databases created before the upgrade therefore need their keys widened.
const TABLES: &[&str] = &[
    "users",
    "channels",
    "videos",
    "comments",
    "song_items",
    "song_diffs",
    "favorites",
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // SQLite integers are already 64-bit.
        if m.get_database_backend() != sea_orm::DatabaseBackend::Postgres {
            return Ok(());
        }

        let db = m.get_connection();
        for table in TABLES {
            // A no-op on databases that already use `bigint`.
            db.execute_unprepared(&format!(
                r#"ALTER TABLE "{table}" ALTER COLUMN "id" TYPE bigint"#
            ))
            .await?;
            // `serial` sequences are `AS integer` and would stop at 2^31 - 1.
            db.execute_unprepared(&format!(
                "DO $$ DECLARE seq text := pg_get_serial_sequence('\"{table}\"', 'id'); \
                 BEGIN IF seq IS NOT NULL THEN EXECUTE format('ALTER SEQUENCE %s AS bigint', seq); \
                 END IF; END $$"
            ))
            .await?;
        }

        // References `song_diffs.id`, so it must match the widened key.
        db.execute_unprepared(
            r#"ALTER TABLE "song_items" ALTER COLUMN "latest_diff_id" TYPE bigint"#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, _m: &SchemaManager) -> Result<(), DbErr> {
        // Narrowing back to `integer` could fail on existing ids and would break the
        // `i64` entities, so this migration is intentionally irreversible in practice.
        Ok(())
    }
}
