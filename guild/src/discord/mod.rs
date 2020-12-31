extern crate serenity;
use serenity::http::Http;
use std::{error::Error, fs};

pub async fn execute_webhook(contents: &str) -> Result<(), Box<dyn Error>> {
    let (id, token) = get_webhook_data()?;

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
                .expect("The URL provided is not a valid Discord Webhook"),
            token,
        )
        .await?;
    Ok(())
}

fn get_webhook_data() -> Result<(u64, String), Box<dyn std::error::Error>> {
    // to determine if the files exist, will fail if they don't and throw an error
    let _ = fs::File::open("id.txt")?;
    let _ = fs::File::open("token.txt")?;

    let id = fs::read_to_string("id.txt").unwrap().parse::<u64>()?;
    let token = fs::read_to_string("token.txt")?;
    Ok((id, token))
}
