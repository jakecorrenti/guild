use clap::ArgMatches;

mod arg_parser;
mod channel;
mod db;
mod discord;

pub async fn run(args: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    match args.subcommand_name() {
        Some("post") => {
            let snippet = arg_parser::retrieve_snippet(args)?;
            discord::execute_webhook(&snippet[..]).await?;
        }
        Some("add") => {
            db::add_channel(args).await?;
        }
        Some("list") => {
            db::list_channels()?;
        }
        Some("remove") => {
            let name = args
                .subcommand_matches("remove")
                .unwrap()
                .value_of("name")
                .unwrap();
            db::remove_channel(name)?;
        }
        Some("rename") => {
            let rename_matches = args.subcommand_matches("rename").unwrap();
            let new = rename_matches.value_of("new-name").unwrap();
            let current = rename_matches.value_of("current-name").unwrap();
            db::rename_channel(&current, &new)?;
        }
        _ => {
            eprintln!("There was no argument provided");
            std::process::exit(1);
        }
    }

    Ok(())
}
