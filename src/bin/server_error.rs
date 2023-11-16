//! Module for the distinct error type, generated by the server.

use std::fmt::{Debug, Display, Formatter};
use std::error::Error;

pub mod prelude {
    pub use super::*;
}

#[derive(Debug)]
pub enum ServerError {

}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "")
        }
    }
}

impl Error for ServerError {}