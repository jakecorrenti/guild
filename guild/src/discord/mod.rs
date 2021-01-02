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
                .expect("You did not provide a valid Discord Webhook URL"),
            token,
        )
        .await?;
    Ok(())
}

fn get_webhook_data() -> Result<(u64, String), Box<dyn Error>> {
    // to determine if the files exist, will fail if they don't and throw an error
    let open_err_message = "There is no Discord Webhook URL set";
    let _ = fs::File::open("id.txt").expect(open_err_message);
    let _ = fs::File::open("token.txt").expect(open_err_message);

    let read_err_message = "There was an error accessing your Discord Webhook information";
    let id = fs::read_to_string("id.txt")
        .unwrap()
        .parse::<u64>()
        .expect(read_err_message);

    let token = fs::read_to_string("token.txt").expect(read_err_message);

    Ok((id, token))
}

pub fn persist_webhook_data(id: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let err_message = "There was an error persisting you're Discord Webhook information";
    fs::write("id.txt", id).expect(err_message);
    fs::write("token.txt", token).expect(err_message);
    Ok(())
}
