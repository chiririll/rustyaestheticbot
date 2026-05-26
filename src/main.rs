use rand::Rng;
use std::{env, sync::Arc};
use teloxide::{
    prelude::*,
    types::{FileId, InputFile},
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

    let stickers: Arc<Vec<FileId>> = Arc::new(
        sticker_set
            .stickers
            .into_iter()
            .map(|sticker| sticker.file.id)
            .collect(),
    );

    assert!(
        !stickers.is_empty(),
        "Sticker set is empty: check STICKER_SET_NAME"
    );

    log::info!("Loaded {} stickers", stickers.len());

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let stickers = Arc::clone(&stickers);
        async move {
            let idx = rand::rng().random_range(0..stickers.len());
            let file_id = stickers[idx].clone();
            bot.send_sticker(msg.chat.id, InputFile::file_id(file_id))
                .await?;
            Ok(())
        }
    })
    .await;
}
