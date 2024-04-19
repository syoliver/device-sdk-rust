
use std::fmt;
use std::vec::Vec;
use std::error::Error;

#[derive(Debug)]
pub struct BootstrapError {
    errors: Vec<Box<dyn Error>>
}

impl BootstrapError {
    pub fn new() -> BootstrapError {
        BootstrapError{
            errors: Vec::new()
        }
    }
    pub fn append(&mut self, error: Box<dyn Error>) {
        self.errors.push(error)
    }

    pub fn contains_error(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Error for BootstrapError {
    fn description(&self) -> &str {
        "Error during bootstrap"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut errors_descriptions = String::new();
        for error in &self.errors {
            errors_descriptions.push_str(&format!("{}\n", error));
        }
        write!(f, "Bootstrap failure:\n{}", errors_descriptions)
    }
}

impl From<Box<dyn std::error::Error + std::marker::Send + Sync>> for BootstrapError {
    fn from(err: Box<dyn std::error::Error + std::marker::Send + Sync>) -> Self {
        let mut bootstrap_err = BootstrapError::new();
        bootstrap_err.append(err);
        bootstrap_err
    }
}
