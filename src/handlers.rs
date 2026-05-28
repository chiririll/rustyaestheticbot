use teloxide::{
    prelude::*,
    types::{
        InlineQuery, InlineQueryResult, InlineQueryResultCachedSticker, InputFile, Message,
    },
};

use crate::sticker_picker::StickerPicker;

const INLINE_RESULTS_LIMIT: usize = 5;

pub async fn handle_message(bot: Bot, msg: Message, picker: StickerPicker) -> ResponseResult<()> {
    let file_id = picker.pick_one();
    bot.send_sticker(msg.chat.id, InputFile::file_id(file_id))
        .await?;
    Ok(())
}

pub async fn handle_inline_query(
    bot: Bot,
    query: InlineQuery,
    picker: StickerPicker,
) -> ResponseResult<()> {
    let results: Vec<InlineQueryResult> = picker
        .pick_many(INLINE_RESULTS_LIMIT)
        .into_iter()
        .enumerate()
        .map(|(i, file_id)| {
            InlineQueryResult::CachedSticker(InlineQueryResultCachedSticker::new(
                i.to_string(),
                file_id,
            ))
        })
        .collect();

    bot.answer_inline_query(query.id, results)
        .cache_time(0)
        .is_personal(true)
        .await?;
    Ok(())
}
