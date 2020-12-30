use super::discord;
use clap::ArgMatches;
use std::{fs, process};

pub async fn set_webhook_info(args: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let url_matches = args.subcommand_matches("set").unwrap();
    let url_contents: Vec<&str> = url_matches.value_of("url").unwrap().split("/").collect();
    let guild_data = &url_contents[url_contents.len() - 2..];

    let id = &guild_data.first().unwrap();
    let token = &guild_data.last().unwrap();

    if let Err(e) = discord::valid_webhook(&id, token).await {
        eprintln!("{}", e);
        process::exit(1);
    }

    persist_webhook_data(&id, &token)?;

    Ok(())
}

pub fn retrieve_snippet(args: &ArgMatches) -> Result<String, Box<dyn std::error::Error>> {
    /*
     * can unwrap these two because they are all required in order for parsing to be valid. If
     * they are not provided, the parser will panick and provide a default error message
     */
    let post_matches = args.subcommand_matches("post").unwrap();
    let filename = post_matches.value_of("filename").unwrap();

    let file = fs::read_to_string(&filename)?;
    let file_contents: Vec<&str> = file.lines().collect();

    let end = post_matches
        .value_of("end")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let start = post_matches
        .value_of("start")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    if start <= 0 || end > file_contents.len() {
        eprintln!("The line range provided is invalid.");
        process::exit(1);
    }

    let mut snippet_lines = Vec::new();
    let filtered = file_contents
        .iter()
        .enumerate()
        .filter(|(idx, _)| (idx + 1) >= start && idx < &end);

    // TODO check if the hl arg is present and push the header accordingly
    snippet_lines.push("```");
    filtered.for_each(|line| snippet_lines.push(&line.1));
    snippet_lines.push("```");

    let mut complete_snippet = String::new();
    snippet_lines.iter().enumerate().for_each(|(idx, line)| {
        complete_snippet.push_str(line);
        if idx != snippet_lines.len() {
            complete_snippet.push('\n');
        }
    });

    Ok(complete_snippet)
}

// TODO
fn determine_hl_type(filename: &str) -> String {
    /*
     * examples:
     * .rs  => rust
     * .c   => c
     * .js  => javascript
     * .cpp => cpp
     */
    String::new()
}

fn persist_webhook_data(id: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write("id.txt", id)?;
    fs::write("token.txt", token)?;
    Ok(())
}
