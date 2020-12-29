extern crate serenity;
use serenity::http::Http;

pub async fn execute_webhook(contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    let id: u64 = 793542017656946692;
    let token = "gGLdKvf9NemG_9ruc7sOgnlZ16Wn7E3i0MXw0J56rfd4iVUKGRHDp7w1sJd5b48LaksT";
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
