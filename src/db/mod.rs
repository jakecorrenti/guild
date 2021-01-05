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

    let channels = list_channels()?;
    channels.iter().for_each(|channel| {
        if channel.name == name {
            eprintln!("The name entered already exists for another Discord channel");
            std::process::exit(1);
        }
    });

    let conn = Connection::open(&CHANNEL_DB[..])?;
    conn.execute(
        "INSERT INTO channels (name, id, token) VALUES (?1, ?2, ?3)",
        &[&name, &id, &token],
    )?;

    Ok(())
}

pub fn list_channels() -> Result<Vec<Channel>> {
    let conn = Connection::open(&CHANNEL_DB[..])?;

    let mut stmt = conn.prepare("SELECT name, id, token FROM channels")?;
    let channels = stmt.query_map(NO_PARAMS, |row| {
        Ok(Channel {
            name: row.get(0)?,
            id: row.get(1)?,
            token: row.get(2)?,
        })
    })?;

    let mut ch = Vec::new();
    channels.for_each(|channel| ch.push(channel.unwrap()));

    Ok(ch)
}

pub fn remove_channel(name: &str) -> Result<()> {
    let conn = Connection::open(&CHANNEL_DB[..])?;
    conn.execute("DELETE FROM channels WHERE name=(?1)", &[name])?;
    Ok(())
}

pub fn rename_channel(current: &str, new: &str) -> Result<()> {
    let conn = Connection::open(&CHANNEL_DB[..])?;
    // TODO check to make sure that the new name doesn't already exist
    conn.execute(
        "UPDATE channels
         SET name=(?2)
         WHERE name=(?1)",
         &[current, new]
    )?;
    Ok(())
}

pub fn get_webhook_data(name: &str) -> Result<(u64, String)> {
    let conn = Connection::open(&CHANNEL_DB[..])?;
    let mut stmt = conn.prepare("SELECT name, id, token FROM channels WHERE name=(?1)")?;
    let channel = stmt.query_row(&[&name], |row| {
        Ok(Channel {
            name: row.get(0)?,
            id: row.get(1)?,
            token: row.get(2)?,
        })
    })?;

    let id_to_u64 = channel.id.parse::<u64>().unwrap();
    Ok((id_to_u64, channel.token))
}
