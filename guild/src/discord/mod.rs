extern crate serenity;
use serenity::http::Http;

pub async fn execute_webhook(contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    let http = Http::new_with_token(token);
    let webhook = http.get_webhook_with_token(id, token).await?;

    webhook
        .execute(&http, false, |w| {
            w.content(contents);
            w
        })
        .await?;

    Ok(())
}
