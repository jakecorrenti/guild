use clap::ArgMatches;

extern crate rusqlite;
use rusqlite::{Connection, Result, NO_PARAMS};

use super::{channel::Channel, discord};

const CHANNEL_DB: &str = "channels.db";

fn create_channels_table() -> Result<()> {
    let conn = Connection::open(&CHANNEL_DB[..])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS channels (name MEDIUMTEXT, id BIGINT, token MEDIUMTEXT)",
        NO_PARAMS,
    )?;
    Ok(())
}

pub async fn add_channel(args: &ArgMatches<'_>) -> Result<()> {
    create_channels_table()?;

    let url_matches = args.subcommand_matches("add").unwrap();
    let url_contents: Vec<&str> = url_matches.value_of("url").unwrap().split("/").collect();
    let guild_data = &url_contents[url_contents.len() - 2..];
    let name = url_matches.value_of("name").unwrap();

    let id = &guild_data.first().unwrap();
    let token = &guild_data.last().unwrap();

    if let Err(e) = discord::valid_webhook(&id, token).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    // TODO Determine if that name has already been used for another channel

    let conn = Connection::open(&CHANNEL_DB[..])?;
    conn.execute(
        "INSERT INTO channels (name, id, token) VALUES (?1, ?2, ?3)",
        &[&name, &id, &token],
    )?;

    Ok(())
}

pub fn list_channels() -> Result<()> {
    let conn = Connection::open(&CHANNEL_DB[..])?;

    let mut stmt = conn.prepare("SELECT name, id, token FROM channels")?;
    let channels = stmt.query_map(NO_PARAMS, |row| {
        Ok(Channel {
            name: row.get(0)?,
            id: row.get(1)?,
            token: row.get(2)?,
        })
    })?;

    channels.for_each(|channel| println!("{:?}", channel.unwrap()));

    Ok(())
}

pub fn remove_channel(name: &str) -> Result<()> {
    let conn = Connection::open(&CHANNEL_DB[..])?;
    conn.execute("DELETE FROM channels WHERE name=(?1)", &[name])?;
    Ok(())
}

pub fn rename_channel(current: &str, new: &str) -> Result<()> {
    let conn = Connection::open(&CHANNEL_DB[..])?;
    conn.execute(
        "UPDATE channels
         SET name=(?2)
         WHERE name=(?1)",
         &[current, new]
    )?;
    Ok(())
}
