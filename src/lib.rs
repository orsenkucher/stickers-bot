pub mod config;
mod ffmpeg;
mod schema;
pub mod util;

pub use config::ConfigParameters;
pub use schema::schema;

use teloxide::adaptors::DefaultParseMode;
use teloxide::dispatching::dialogue::{serializer::Json, ErasedStorage, SqliteStorage, Storage};
use teloxide::prelude::*;
use teloxide::requests::RequesterExt;
use teloxide::types::ParseMode;

use std::error::Error;

type HandlerError = Box<dyn Error + Send + Sync>;
type HandlerResult = Result<(), HandlerError>;

type MyBot = AutoSend<DefaultParseMode<Bot>>;

type MyHandler = dptree::Handler<
    'static,
    DependencyMap,
    HandlerResult,
    teloxide::dispatching::DpHandlerDescription,
>;

pub fn bot_from_env() -> MyBot {
    Bot::from_env().parse_mode(ParseMode::Html).auto_send()
}

pub async fn dispatch(bot: MyBot, parameters: ConfigParameters, handler: MyHandler) {
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![parameters])
        .default_handler(|upd| async move {
            tracing::warn!("unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "an error has occurred in the dispatcher",
        ))
        // .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
