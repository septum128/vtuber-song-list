use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "favorites",
            &[
                ("id", ColType::PkAuto),
                ("user_id", ColType::BigInteger),
                ("song_item_id", ColType::BigInteger),
            ],
            &[("users", "user_id"), ("song_items", "song_item_id")],
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_favorites_on_user_id")
                .table(Alias::new("favorites"))
                .col(Alias::new("user_id"))
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("index_favorites_on_user_id_and_song_item_id")
                .table(Alias::new("favorites"))
                .col(Alias::new("user_id"))
                .col(Alias::new("song_item_id"))
                .unique()
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "favorites").await?;
        Ok(())
    }
}
