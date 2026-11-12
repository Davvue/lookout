pub mod file;

use std::io::Error;
use crate::config::{Config, OutputList, Service};

pub trait Store {
    fn config(&self) -> Result<Config, StoreError>;
    fn services(&self) -> Result<Vec<Service>, StoreError>;
    fn output(&self) -> Result<OutputList, StoreError>;
    fn save_output(&self, output: &OutputList) -> Result<(), StoreError>;
    fn save_services(&self, services: &[Service]) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
    Data(Box<dyn std::error::Error + Send + Sync>)
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StoreError::Io(e) => write!(f, "storage I/O error: {e}"),
            StoreError::Data(e) => write!(f, "invalid stored data: {e}"),
        }
    }
}
impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(value: Error) -> Self { StoreError::Io(value) }
}

impl From<serde_json::Error> for StoreError {
    fn from(value: serde_json::Error) -> Self {
        StoreError::Data(Box::new(value))
    }
}