#[cfg(test)]
mod tests {
    use super::super::config::Config;
    use color_eyre::eyre::Result;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.source, PathBuf::from("."));
        assert_eq!(config.destination, PathBuf::from("johnny-d"));
    }

    #[test]
    fn test_save_and_load() -> Result<()> {
        // Create a temporary directory for test
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create a config with custom values
        let config = Config {
            source: PathBuf::from("/some/source"),
            destination: PathBuf::from("/some/destination"),
        };

        // Save the config
        config.save(dir_path)?;

        // Check that the file exists
        let config_path = dir_path.join("johnny-d-config.json");
        assert!(config_path.exists());

        // Load the config and verify it matches
        let loaded_config = Config::try_load(dir_path)?;
        assert_eq!(loaded_config, config);

        Ok(())
    }

    #[test]
    fn test_load_nonexistent() {
        // Create a temporary directory for test
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Try to load from a directory without a config file
        let result = Config::try_load(dir_path);
        assert!(result.is_err());
    }
}
