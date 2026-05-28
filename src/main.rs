mod handlers;
mod sticker_picker;

use std::env;

use teloxide::{prelude::*, types::Update};

use crate::{
    handlers::{handle_inline_query, handle_message},
    sticker_picker::StickerPicker,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting sticker bot...");

    let sticker_set_name = env::var("STICKER_SET_NAME").expect("STICKER_SET_NAME must be set");
    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");

    let bot = Bot::new(bot_token);
    let sticker_set = bot
        .get_sticker_set(sticker_set_name)
        .await
        .expect("Failed to fetch sticker set");

    let picker = StickerPicker::new(
        sticker_set
            .stickers
            .into_iter()
            .map(|sticker| sticker.file.id)
            .collect(),
    );
    log::info!("Loaded {} stickers", picker.len());

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_inline_query().endpoint(handle_inline_query));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![picker])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
