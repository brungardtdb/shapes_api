use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// An error that warns the user of a missing
/// field on a shape when building that shape
pub struct MissingPropertyError {
    /// The name of the missing shape property
    pub property_name: &'static str,
}

impl MissingPropertyError {
    /// Create a [MissingPropertyError] from a static string
    /// containing the name of the shape's missing property
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
