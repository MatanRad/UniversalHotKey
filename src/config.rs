use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const DEFAULT_CONFIG_PATH: &'static str = "/etc/uhk.json";

// TODO: Add Logfile
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct UHKConfig {
    pub scripts: Vec<String>,
}

impl UHKConfig {
    pub fn new(path: &str) -> Result<Self> {
        let conf = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(conf.as_str())?)
    }

    pub fn default() -> Result<Self> {
        UHKConfig::new(DEFAULT_CONFIG_PATH)
    }
}
