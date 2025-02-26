use color_eyre::eyre::{Result, eyre};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration for the Johnny Decimal system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    /// The source directory to organize files from
    pub source: PathBuf,

    /// The destination directory to organize files into
    pub destination: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source: PathBuf::from("."),
            destination: PathBuf::from("johnny-d"),
        }
    }
}

impl Config {
    const CONFIG_FILENAME: &'static str = "johnny-d-config.json";

    /// Try to load configuration from the specified directory
    pub fn try_load<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let config_path = dir.as_ref().join(Self::CONFIG_FILENAME);

        if !config_path.exists() {
            return Err(eyre!("Config file not found at {}", config_path.display()));
        }

        let config_content = fs::read_to_string(&config_path)?;
        let config = serde_json::from_str(&config_content)?;

        Ok(config)
    }

    /// Save configuration to the specified directory
    pub fn save<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let config_path = dir.as_ref().join(Self::CONFIG_FILENAME);

        let config_content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_content)?;

        Ok(())
    }
}
