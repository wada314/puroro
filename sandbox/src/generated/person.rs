//! Hand-written code for Person message.
//!
//! This represents our ideal API for the generated code.

use puroro::{error::Error, Message};

/// A simple person message for testing basic protobuf features
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub email: String,
}

impl Person {
    /// Creates a new Person with default values.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            age: 0,
            email: String::new(),
        }
    }
}

impl Default for Person {
    fn default() -> Self {
        Self::new()
    }
}

impl Message for Person {
    fn parse_from_bytes(_bytes: &[u8]) -> Result<Self, Error> {
        // TODO: Implement actual parsing
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>, Error> {
        // TODO: Implement actual serialization
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        // TODO: Implement actual size computation
        todo!("Size computation not yet implemented")
    }
}
