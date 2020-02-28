use anyhow::anyhow;
use reqwest;
use reqwest::Url;
use std::env;

use crate::telegram::outbound::{AnswerInlineQuery, SendMediaGroup, SendMessage, SendPhoto};

const BASE_URL: &str = "https://api.telegram.org/";

pub async fn answer_inline_query(answer: &AnswerInlineQuery) -> anyhow::Result<()> {
    let url = format!("{}{}/answerInlineQuery", BASE_URL, make_auth()?);
    let endpoint = Url::parse(&url).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(endpoint).json(answer).send().await?;

    if !res.status().is_success() {
        println!(
            "[ERROR] Telegram API: HTTP {}: {:?}",
            res.status(),
            res.text().await,
        )
    }
    Ok(())
}

pub async fn send_message(msg: &SendMessage) -> anyhow::Result<()> {
    let url = format!("{}{}/sendMessage", BASE_URL, make_auth()?);
    let endpoint = Url::parse(&url).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(endpoint).json(msg).send().await?;

    if !res.status().is_success() {
        println!(
            "[ERROR] Telegram API: HTTP {}: {:?}",
            res.status(),
            res.text().await,
        )
    }
    Ok(())
}

pub async fn send_photo(msg: &SendPhoto) -> anyhow::Result<()> {
    let url = format!("{}{}/sendPhoto", BASE_URL, make_auth()?);
    let endpoint = Url::parse(&url).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(endpoint).json(msg).send().await?;

    if !res.status().is_success() {
        println!(
            "[ERROR] Telegram API: HTTP {}: {:?}",
            res.status(),
            res.text().await,
        )
    }
    Ok(())
}

pub async fn send_multi_photo(msg: &SendMediaGroup) -> anyhow::Result<()> {
    let url = format!("{}{}/sendMediaGroup", BASE_URL, make_auth()?);
    let endpoint = Url::parse(&url).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(endpoint).json(msg).send().await?;

    if !res.status().is_success() {
        println!(
            "[ERROR] Telegram API: HTTP {}: {:?}",
            res.status(),
            res.text().await,
        )
    }
    Ok(())
}

fn make_auth() -> anyhow::Result<String> {
    let api_key =
        env::var_os("TELEGRAM_BOT_TOKEN").ok_or(anyhow!("TELEGRAM_BOT_TOKEN was not set"))?;

    Ok(format!(
        "bot{}",
        api_key
            .to_str()
            .ok_or(anyhow!("TELEGRAM_BOT_TOKEN could not be decoded as String"))?
    ))
}
