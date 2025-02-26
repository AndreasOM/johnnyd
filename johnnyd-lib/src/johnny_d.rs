use color_eyre::eyre::Result;

/// Represents a Johnny Decimal system organizer
///
/// The Johnny Decimal system is a methodology for organizing files and folders
/// into a structured system of areas (10-19, 20-29, etc.) and categories.
/// See https://johnnydecimal.com/ for more details.
pub struct JohnnyD {
    /// Base directory for organizing files
    base_dir: std::path::PathBuf,
}

impl JohnnyD {
    /// Create a new JohnnyD organizer with the specified base directory
    pub fn new<P: AsRef<std::path::Path>>(base_dir: P) -> Self {
        tracing::debug!("Creating new JohnnyD organizer with base directory: {:?}", base_dir.as_ref());
        JohnnyD {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Initialize the Johnny Decimal structure in the base directory
    pub fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing Johnny Decimal structure in {:?}", self.base_dir);
        
        // Create base directory if it doesn't exist
        if !self.base_dir.exists() {
            tracing::debug!("Creating base directory");
            std::fs::create_dir_all(&self.base_dir)?;
        }
        
        Ok(())
    }

    /// Returns the base directory of this JohnnyD organizer
    pub fn base_dir(&self) -> &std::path::Path {
        &self.base_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new_johnny_d() {
        let jd = JohnnyD::new("/tmp/test");
        assert_eq!(jd.base_dir(), Path::new("/tmp/test"));
    }
}