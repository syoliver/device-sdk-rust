use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct OsStringConversionError;

impl fmt::Display for OsStringConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to convert OsString to String")
    }
}

impl Error for OsStringConversionError {}

