use dotenv::dotenv;
use video_stickers::{util, ConfigParameters};

#[tokio::main]
async fn main() {
    dotenv().ok();
    util::setup_tracing();
    tracing::info!("video stickers bot is starting");

    let maintainers = vec![
        364448153, // Orsen
    ];
    let parameters = ConfigParameters::new(maintainers);

    let bot = video_stickers::bot_from_env();
    let handler = video_stickers::schema();

    video_stickers::dispatch(bot, parameters, handler).await;
}
