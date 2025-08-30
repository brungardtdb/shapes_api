use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MissingPropertyError {
    pub property_name: &'static str,
}

impl MissingPropertyError {
    pub fn from(property_name: &'static str) -> Self {
        MissingPropertyError { property_name }
    }
}

impl fmt::Display for MissingPropertyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let property = self.property_name;
        write!(f, "The required property {property} was missing.")
    }
}

impl Error for MissingPropertyError {}