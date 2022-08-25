use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TomlqError(pub String);

impl fmt::Display for TomlqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for TomlqError {}
