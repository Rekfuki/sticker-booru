use crate::scryfall::models::SearchResult;
use crate::telegram::outbound::{
    AnswerInlineQuery, InlineQueryResultCachedSticker, InputTextMessageContent,
};

pub fn search_results_to_inline_query_response(
    query_id: String,
    search_result: &SearchResult,
) -> AnswerInlineQuery {
    AnswerInlineQuery {
        inline_query_id: query_id,
        results: match &search_result.data {
            Some(cards) => cards
                .iter()
                .take(50) // Max 50 results in response
                .map(|res| unimplemented!()) // sticker_to_inline_sticker
                .collect(),
            None => Vec::new(),
        },
    }
}

// fn sticker_to_inline_sticker(card: &Card) -> InlineQueryResultCachedSticker {
//     let thumbnail = match &card.image_uris {
//         Some(uris) => uris.get("art_crop").or(uris.get("small")).cloned(),
//         None => None,
//     };

//     InlineQueryResultCachedSticker {
//         query_result_type: String::from("sticker"),
//         id: card.id.clone(),
//         title: card.name.clone(),
//         url: Some(card.scryfall_uri.clone()),
//         description: Some(card.oracle_text.clone().unwrap_or(card.type_line.clone())),
//         thumb_url: thumbnail,
//         hide_url: Some(true),
//         input_message_content: InputTextMessageContent {
//             message_text: card.scryfall_uri.clone(),
//             disable_web_page_preview: false,
//         },
//     }
// }
