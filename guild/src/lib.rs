use clap::ArgMatches;

mod discord; 
mod arg_parser;

pub async fn run(args: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let snippet = arg_parser::retrieve_snippet(args)?;
    discord::execute_webhook(&snippet[..]).await?;
    Ok(())
}
