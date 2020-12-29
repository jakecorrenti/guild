use clap::ArgMatches;
use std::{fs, process};

pub fn retrieve_snippet(args: &ArgMatches) -> Result<String, Box<dyn std::error::Error>> {
    /*
     * can unwrap these four because they are all required in order for parsing to be valid. If
     * they are not provided, the parser will panick and provide a default error message
     */
    let post_matches = args.subcommand_matches("post").unwrap();
    let filename = post_matches.value_of("filename").unwrap();
    let start = post_matches.value_of("start").unwrap();
    let end = post_matches.value_of("end").unwrap();

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
        if (idx + 1) >= start_as_int as usize && idx < end_as_usize {
            if (idx + 1) == start_as_int as usize {
                if args.is_present("hl") {
                    // TODO
                } else {
                    snippet_lines.push("```");
                }
            }
            snippet_lines.push(line);
            if idx == (end_as_usize - 1) {
                snippet_lines.push("```");
            }
        }
    }

    let mut complete_snippet = String::new();

    for (idx, line) in snippet_lines.iter().enumerate() {
        complete_snippet.push_str(line);
        if idx != snippet_lines.len() {
            complete_snippet.push('\n');
        }
    }

    Ok(complete_snippet)
}

// TODO
fn determine_hl_type(filename: &str) -> String {
    /*
     * examples:
     * .rs => Rust
     * .c => C
     * .js => JavaScript
     * .cpp => C++
     */
    String::new()
}
