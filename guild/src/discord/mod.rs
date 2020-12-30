extern crate serenity;
use serenity::http::Http;

pub async fn execute_webhook(contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: retrieve the token and id values from the persisted files. if they don't exist kill
    // the process and throw an error
    let id: u64 = 793574437340577803;
    let token = "Ra_txBa5RMTlZyZfviTtZIGlsjAvfNrOnPhaaPJmf62sq1EJFXznGGAGu4CZCHm6Tdpj";

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

pub async fn valid_webhook(id: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
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
