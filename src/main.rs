#[macro_use]
extern crate clap;

// Std
use std::process;

// External
use clap::{Arg, Command};

// Internal
use tomlq_rs::Param;

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
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

    let param = Param::new(&matches).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
    });

    if let Err(e) = tomlq_rs::parse(param) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
