extern crate serenity;
use serenity::http::Http;
use std::error::Error;
use super::db;

pub async fn execute_webhook(name: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    let (id, token) = db::get_webhook_data(&name)?;

    let http = Http::new_with_token(&token[..]);
    let webhook = http.get_webhook_with_token(id, &token[..]).await?;

    webhook
        .execute(&http, false, |w| {
            w.content(contents);
            w
        })
        .await?;

    Ok(())
}

pub async fn valid_webhook(id: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let http = Http::new_with_token(token);
    let _ = http
        .get_webhook_with_token(
            id.parse::<u64>()
                .expect("You did not provide a valid Discord Webhook URL"),
            token,
        )
        .await?;
    Ok(())
}
