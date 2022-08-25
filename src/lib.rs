// Std
use std::error::Error;
use std::fs;
use std::io;

// External
use clap::ArgMatches;
use toml::ser;
use toml::Value;

// Internal
use crate::error::TomlqError;

mod error;

#[cfg(test)]
mod test;

/// A structure to store parameters
pub struct Param {
    pub key: String,
    pub filename: String,
}

impl Param {
    /// Creates a new instance of an Param.
    ///
    /// # Examples
    ///
    /// ```
    /// # use clap::{Command, Arg};
    /// # use tomlq_rs::Param;
    /// let matches = Command::new("myapp")
    ///                 .arg(
    ///                     Arg::with_name("KEY")
    ///                         .help("Key to query from the TOML file")
    ///                         .required(true)
    ///                         .index(1),
    ///                     )
    ///                 .arg(
    ///                     Arg::with_name("FILE")
    ///                         .help("A TOML file to load")
    ///                         .required(true)
    ///                         .index(2),
    ///                     )
    ///                 .get_matches_from(vec!["myapp", "key", "a.toml"]);
    ///
    /// let param = Param::new(&matches).unwrap();
    /// assert_eq!(param.key, "key");
    /// assert_eq!(param.filename, "a.toml");
    /// ```
    pub fn new(matches: &ArgMatches) -> Result<Param, &'static str> {
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
        Ok(Param { key, filename })
    }
}

/// Parse toml file
///
/// # Example
///
/// ```
/// # use tomlq_rs::{Param, parse};
/// let param = Param {
///     key: String::from("package.name"),
///     filename: String::from("Cargo.toml")
/// };
/// let result = parse(param).unwrap();
/// assert_eq!(result, ());
/// ```
pub fn parse(param: Param) -> Result<(), Box<dyn Error>> {
    let toml_string = match fs::read_to_string(&param.filename) {
        Ok(c) => c,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    return Err(Box::new(TomlqError(String::from("No such file!"))));
                }
                _ => {
                    return Err(Box::new(TomlqError(String::from("Cannot read the file!"))));
                }
            };
        }
    };

    let value = query_toml_value(&toml_string, &param.key)?;
    println!("{}", value);

    Ok(())
}

/// Query value from toml by key.
///
/// # Examples
///
/// ```
/// # use tomlq_rs::query_toml_value;
/// let toml_str = r#"key = "value"  # This is a comment at the end of a line"#;
/// let key = "key";
/// let value = query_toml_value(toml_str, key).unwrap();
/// assert_eq!(value, "value");
/// ```
pub fn query_toml_value(toml_str: &str, key: &str) -> Result<String, String> {
    let toml_obj = match toml::from_str(toml_str) {
        Ok(v) => v,
        Err(_) => return Err(String::from("Parsing failed!")),
    };

    let value = parse_key(key)
        .iter()
        .fold(
            Some(&toml_obj),
            |accumulator: Option<&Value>, key| match accumulator {
                Some(a) => a.get(key),
                None => None,
            },
        );

    match value {
        // TODO: 添加选项输出 ser::to_string_pretty()
        Some(v) => match ser::to_string(v) {
            Ok(s) => match v.type_str() {
                "string" => Ok(format!("{}", &s[1..s.len() - 1])),
                _ => Ok(format!("{}", s)),
            },
            Err(_) => Err(format!("Key {} serialization failed!", key)),
        },
        None => Err(format!("Key {} not found!", key)),
    }
}

/// Parse key.
///
/// # Examples
///
/// ```
/// # use tomlq_rs::parse_key;
/// let key = r#"site."google.com""#;
/// let result = parse_key(key);
/// assert_eq!(result, vec!["site", "google.com"]);
/// ```
pub fn parse_key(key: &str) -> Vec<&str> {
    let mut flag: char = '.';
    let mut index: usize = 0;
    let mut key_vector: Vec<&str> = Vec::new();

    for (i, s) in key.chars().enumerate() {
        match s {
            '.' => match flag {
                '.' => {
                    key_vector.push(key[index..i].trim_matches('"').trim_matches('\''));
                    index = i + 1;
                }
                _ => (),
            },
            '\'' => match flag {
                '\'' => flag = '.',
                '.' => flag = '\'',
                _ => (),
            },
            '"' => match flag {
                '"' => flag = '.',
                '.' => flag = '"',
                _ => (),
            },
            _ => (),
        }
    }

    key_vector.push(key[index..key.len()].trim_matches('"').trim_matches('\''));
    key_vector
}
