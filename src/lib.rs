use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

use clap::ArgMatches;
use toml::Value;

pub struct Config {
    pub key: String,
    pub filename: String,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Result<Config, &'static str> {
        let key = match matches.value_of("KEY") {
            Some(f) => f.to_string(),
            None => {
                return Err("Must specify Key to query!");
            }
        };
        let filename = match matches.value_of("FILE") {
            Some(f) => f.to_string(),
            None => {
                return Err("Must specify File to load!");
            }
        };
        Ok(Config { key, filename })
    }
}

#[derive(Debug)]
struct TomlqError(String);

impl fmt::Display for TomlqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for TomlqError {}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match fs::read_to_string(&config.filename) {
        Ok(c) => c,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    return Err(Box::new(TomlqError("No such file".to_string())));
                }
                _ => {
                    return Err(Box::new(TomlqError("Cannot read the file".to_string())));
                }
            };
        }
    };

    let toml_obj = match toml::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            return Err(Box::new(TomlqError("Parsing failed".to_string())));
        }
    };

    let value = config
        .key
        .split('.')
        .fold(
            Some(&toml_obj),
            |accumulator: Option<&Value>, key| match accumulator {
                Some(a) => a.get(key),
                None => None,
            },
        );

    match value {
        Some(v) => {
            println!("{}", v.to_string().trim_matches('"'));
        }
        None => {
            return Err(Box::new(TomlqError(format!(
                "Key {} not found!",
                config.key
            ))));
        }
    };

    Ok(())
}
