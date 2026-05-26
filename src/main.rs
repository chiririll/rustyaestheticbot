use teloxide::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::new(env::var("BOT_TOKEN").expect("BOT_TOKEN must be set"));

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
