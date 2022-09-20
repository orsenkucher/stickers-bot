use std::path::Path;

use teloxide::dispatching::{MessageFilterExt, UpdateFilterExt};
use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::types::{FileMeta, InputFile, VideoNote};

use crate::{ffmpeg, HandlerResult, MyBot, MyHandler};

pub fn schema() -> MyHandler {
    let messages = text_branch();
    let video = video_branch();

    dptree::entry().branch(Update::filter_message().branch(video).branch(messages))
}

fn text_branch() -> MyHandler {
    Message::filter_text().endpoint(text)
}

fn video_branch() -> MyHandler {
    dptree::filter_map(move |input: Message| input.video_note().map(ToOwned::to_owned))
        .endpoint(video)
}

pub async fn text(bot: MyBot, msg: Message) -> HandlerResult {
    tracing::info!("text handler");
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAIE7mMo9r5OA1kYvDqHYhJs8AGlOHJGAAJ0HQACgEAQSTg90lSP8b8hKQQ",
        ),
    )
    .await?;
    Ok(())
}

pub async fn video(bot: MyBot, msg: Message, video: VideoNote) -> HandlerResult {
    use teloxide::types::File as TgFile;
    use tokio::fs::{self, File};

    tracing::info!("video handler");
    if video.duration > 3 {
        alert_will_trim(&bot, &msg).await?;
    }

    tracing::trace!("downloading file: {}", video.file_id);
    let TgFile {
        file_path,
        meta: FileMeta { file_unique_id, .. },
    } = bot.get_file(video.file_id).send().await?;

    fs::create_dir_all("./source").await?;
    let mut file = File::create(format!("./source/{}.mp4", file_unique_id)).await?;
    bot.download_file(&file_path, &mut file).await?;
    tracing::trace!("download completed");

    bot.send_message(msg.chat.id, "Працюю👌").await?;

    let dest = ffmpeg::run(&file_unique_id).await?;

    tracing::trace!("destination file: {}", dest);

    bot.send_document(msg.chat.id, InputFile::file(Path::new(&dest)))
        .await?;

    Ok(())
}

async fn alert_will_trim(bot: &MyBot, msg: &Message) -> HandlerResult {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAIE92Mo-XHn3CzY4Srsx_beoEmRMwmmAALMEAACMLH4SbTPJxD_OvajKQQ",
        ),
    )
    .await?;
    bot.send_message(
        msg.chat.id,
        "Відео задовге для тг, тому буде обрізано до 3 сек.",
    )
    .await?;

    Ok(())
}
