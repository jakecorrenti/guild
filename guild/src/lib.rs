use clap::ArgMatches;
use std::{fs, process};

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {

    /*
     * can unwrap these four because they are all required in order for parsing to be valid. If
     * they are not provided, the parser will panick and provide a default error message
     */
    let post_matches = args.subcommand_matches("post").unwrap();
    let filename = post_matches.value_of("filename").unwrap();
    let start = post_matches.value_of("start").unwrap();
    let end = post_matches.value_of("end").unwrap();

    // TODO 
    if let Some(should_highlight) = post_matches.value_of("hl") {}

    let file = fs::read_to_string(&filename)?;
    let file_contents_per_line: Vec<&str> = file.lines().collect();
    let start_as_int = start.parse::<i32>().unwrap();
    let end_as_usize = end.parse::<usize>().unwrap();

    if start_as_int <= 0 || end_as_usize > file_contents_per_line.len() {
        eprintln!("The line range provided is invalid.");
        process::exit(1);
    }

    let mut snippet_lines = Vec::new();
    for (idx, line) in file_contents_per_line.iter().enumerate() {
        if idx < end_as_usize {
            snippet_lines.push(line);
        }
    }

    for line in snippet_lines {
        println!("{}", line);
    }

    Ok(())
}
