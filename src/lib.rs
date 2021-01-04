use clap::ArgMatches;

mod arg_parser;
mod discord;

pub async fn run(args: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    match args.subcommand_name() {
        Some("post") => {
            let snippet = arg_parser::retrieve_snippet(args)?;
            discord::execute_webhook(&snippet[..]).await?;
        }
        Some("set") => arg_parser::set_webhook_info(args).await?,
        _ => {
            eprintln!("There was no argument provided");
            std::process::exit(1);
        },
    }

    Ok(())
}
