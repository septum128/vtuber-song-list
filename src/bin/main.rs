use loco_rs::cli;
use migration::Migrator;
use vtuber_song_list::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
