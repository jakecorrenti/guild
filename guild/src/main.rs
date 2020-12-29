#[macro_use]
extern crate clap;
use clap::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    guild::run(&matches)
}
