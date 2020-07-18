#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]
mod convert;
mod db;
mod platform;
mod telegram;

use anyhow::{anyhow, Context};
use bb8_postgres::{
    bb8::{Pool, RunError},
    PostgresConnectionManager,
};
use diesel::pg::PgConnection;
use once_cell::sync::OnceCell;
use platform::Platform;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::future::Future;
use std::str::FromStr;
use std::time::Duration;
use telegram::{
    inbound::{MessageEntityType, TelegramUpdate},
    outbound::{InputMediaPhoto, ParseMode, SendMediaGroup, SendMessage, SendPhoto},
};
use warp::Filter;

static DB_POOL: OnceCell<
    bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let platform = platform::Platform::autodetect();
    let postgres_uri = platform.get_database_config()?.as_uri();

    let config = tokio_postgres::config::Config::from_str(&postgres_uri).unwrap();
    let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

    let pool = Pool::builder()
        .build(pg_mgr)
        .await
        .context("pool builder error")?;

    DB_POOL
        .set(pool)
        .map_err(|_| anyhow!("failed to initialise DB pool"))?;

    platform.serve(process_event).await
}

async fn process_event(update: TelegramUpdate) -> anyhow::Result<Value> {
    // println!("{:?}", msg_body);

    if update.inline_query.is_some() {
        handle_inline_query(&update).await?; // todo handle error
    }

    if update.message.is_some() {
        handle_message(&update).await?;
    }

    Ok(Value::default())
}

async fn handle_inline_query(update: &TelegramUpdate) -> anyhow::Result<()> {
    let q = update.inline_query.as_ref().unwrap();

    if q.query.len() == 0 {
        return Ok(());
    }

    let pool = DB_POOL.get().context("failed to get pool")?;
    let conn = pool.get();

    // let results = sticker_search(&q.query, "name", 1).unwrap();
    // let results = unimplemented!();
    let results = Vec::new();
    let response = convert::search_results_to_inline_query_response(q.id.clone(), results);
    telegram::api::answer_inline_query(&response).await
}

async fn handle_message(update: &TelegramUpdate) -> anyhow::Result<()> {
    let msg = update.message.as_ref().unwrap();

    // Handle any [[ ]] references before checking for commands etc.
    handle_plaintext(update).await?;

    if msg.entities.is_none() {
        return Ok(());
    }

    let entities = msg.entities.as_ref().unwrap();

    if entities.len() == 0 {
        return Ok(());
    }

    let msg_text = msg.text.as_ref().unwrap();

    for cmd in entities
        .iter()
        .filter(|e| e.entity_type == MessageEntityType::BotCommand)
    {
        let command_txt: String = msg_text
            .chars()
            .into_iter()
            .skip(cmd.offset as usize)
            .take(cmd.length as usize)
            .collect();

        if &command_txt == "/start" {
            telegram::api::send_message(&SendMessage {
                chat_id: msg.chat.id.clone(),
                parse_mode: Some(ParseMode::Markdown),
                disable_web_page_preview: Some(true),
                text: "Welcome to StickerBooruBot!

*Usage*
StickerBooruBot works in both _inline_ mode and in active mode.
Inline mode means you just tag @StickerBooruBot and start typing while the results show up above your keyboard.
Tapping a result will send it in your chat.
*Questions, Improvements, Changes*
StickerBooruBot is open source and lives on [Github here](https://github.com/Rekfuki/sticker-booru).
It was originally forked from ScryfallBot, [Github here](https://github.com/OliverHofkens/scryfall-telegram-rs-serverless).
If you have a great idea, feature request, or bug report, feel free to [open an issue here](https://github.com/Rekfuki/sticker-booru/issues)

*Legal stuff*
- The code for this bot is licensed under the [MIT License](https://github.com/Rekfuki/sticker-booru/blob/master/LICENSE), so you're free to change it!
                ".to_string(),
            }).await?;
        } else {
            println!("Unsupported command: {}", command_txt);
        }
    }
    Ok(())
}

async fn handle_plaintext(update: &TelegramUpdate) -> anyhow::Result<()> {
    let msg = &update.message.as_ref().unwrap();
    let msg_text = msg.text.to_owned();

    if msg_text.is_none() {
        return Ok(());
    }
    let msg_text = msg_text.unwrap();

    let re = Regex::new(r"\[\[(.+?)\]\]").unwrap();

    let results: Vec<String> = re
        .captures_iter(&msg_text)
        .filter_map(|cap| unimplemented!())
        .collect();

    if results.len() == 0 {
        return Ok(());
    }

    if results.len() == 1 {
        telegram::api::send_photo(&SendPhoto {
            chat_id: msg.chat.id.clone(),
            photo: results[0].to_owned(),
            caption: None,
            parse_mode: None,
        })
        .await
    } else {
        telegram::api::send_multi_photo(&SendMediaGroup {
            chat_id: msg.chat.id.clone(),
            media: results
                .iter()
                .map(|x| InputMediaPhoto {
                    media_type: String::from("photo"),
                    media: x.to_owned(),
                    caption: None,
                    parse_mode: None,
                })
                .collect(),
        })
        .await
    }
}
