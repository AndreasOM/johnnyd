use crate::config::Config;
use color_eyre::eyre::Result;
use std::path::{Path, PathBuf};

/// Represents a Johnny Decimal system organizer
///
/// The Johnny Decimal system is a methodology for organizing files and folders
/// into a structured system of areas (10-19, 20-29, etc.) and categories.
/// See https://johnnydecimal.com/ for more details.
pub struct JohnnyD {
    /// Base directory where the config is located
    base_dir: PathBuf,
    /// Configuration for the Johnny Decimal system
    config: Config,
    /// Flag to indicate if this is the first run (config was not found)
    first_run: bool,
}

impl JohnnyD {
    /// Create a new JohnnyD organizer with the specified base directory
    /// Attempts to load the config from the base directory, or falls back to default
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        let base_path = base_dir.as_ref().to_path_buf();

        tracing::debug!(
            "Creating new JohnnyD organizer with base directory: {:?}",
            base_path
        );

        // Try to load config from the base directory
        let (config, first_run) = match Config::try_load(&base_path) {
            Ok(config) => {
                tracing::debug!("Loaded existing config from {:?}", base_path);
                (config, false)
            }
            Err(err) => {
                tracing::debug!("Failed to load config: {}, using default", err);
                (Config::default(), true)
            }
        };

        JohnnyD {
            base_dir: base_path,
            config,
            first_run,
        }
    }

    /// Initialize the Johnny Decimal structure in the destination directory
    pub fn initialize(&self) -> Result<()> {
        // Combine base path with destination path from config
        let dest_path = self.base_dir.join(&self.config.destination);

        tracing::info!("Initializing Johnny Decimal structure in {:?}", dest_path);

        // Create destination directory if it doesn't exist
        if !dest_path.exists() {
            tracing::debug!("Creating destination directory");
            std::fs::create_dir_all(&dest_path)?;
        }

        // If this is the first run, save the config
        if self.first_run {
            tracing::debug!("First run, saving default config");
            self.config.save(&self.base_dir)?;
        }

        Ok(())
    }

    /// Returns the base directory of this JohnnyD organizer
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// Returns whether this is the first run (config was not found)
    pub fn first_run(&self) -> bool {
        self.first_run
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_new_johnny_d() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Test with a directory that doesn't have a config file
        let jd = JohnnyD::new(temp_path);
        assert_eq!(jd.base_dir(), temp_path);
        assert!(jd.first_run());

        // Initialize to create the config file
        jd.initialize().unwrap();

        // Create a new JohnnyD with the same path, should load the config
        let jd2 = JohnnyD::new(temp_path);
        assert_eq!(jd2.base_dir(), temp_path);
        assert!(!jd2.first_run());
    }
}
