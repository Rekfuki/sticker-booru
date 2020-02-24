use crate::db::models::Sticker;
use crate::telegram::outbound::{
    AnswerInlineQuery, InlineQueryResultCachedSticker, InputTextMessageContent,
};

pub fn search_results_to_inline_query_response(
    query_id: String,
    query_result: impl IntoIterator<Item = Sticker>,
) -> AnswerInlineQuery {
    AnswerInlineQuery {
        inline_query_id: query_id,
        results: {
            query_result
                .into_iter()
                .take(50) // Max 50 results in response
                .map(|res| unimplemented!()) // sticker_to_inline_sticker
                .collect()
        },
    }
}
