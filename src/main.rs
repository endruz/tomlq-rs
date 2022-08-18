use std::process;

use clap::{App, Arg};

use tomlq::Config;

fn main() {
    let matches = App::new("tomlq")
        .version("0.1.0")
        .author("endruz <endruz@foxmail.com>")
        .about("A command-line TOML processing tool.")
        .arg(
            Arg::with_name("KEY")
                .help("Key to query from the TOML file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILE")
                .help("A TOML file to load")
                .required(true)
                .index(2),
        )
        .get_matches();

    let config = Config::new(&matches).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
    });

    if let Err(e) = tomlq::run(config) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
