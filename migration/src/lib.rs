#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;
mod m20260318_000001_add_kind_to_users;
mod m20260318_000002_create_channels;
mod m20260318_000003_create_videos;
mod m20260318_000004_create_comments;
mod m20260318_000005_create_song_items;
mod m20260318_000006_create_song_diffs;
mod m20260319_000007_create_favorites;

mod m20260328_022428_add_icon_url_to_channels;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260318_000001_add_kind_to_users::Migration),
            Box::new(m20260318_000002_create_channels::Migration),
            Box::new(m20260318_000003_create_videos::Migration),
            Box::new(m20260318_000004_create_comments::Migration),
            Box::new(m20260318_000005_create_song_items::Migration),
            Box::new(m20260318_000006_create_song_diffs::Migration),
            Box::new(m20260319_000007_create_favorites::Migration),
            Box::new(m20260328_022428_add_icon_url_to_channels::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
