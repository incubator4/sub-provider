use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub groups: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        toml::from_str(
            std::fs::read_to_string(path)
                .map_err(|e| Error::Io(e))?
                .as_str(),
        )
        .map_err(|e| Error::Toml(e))
    }
}
