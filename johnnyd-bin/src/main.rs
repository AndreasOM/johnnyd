use color_eyre::eyre::Result;
use johnnyd_lib::JohnnyD;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn setup_tracing() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        // Set default log level if RUST_LOG is not set
        unsafe {
            std::env::set_var("RUST_LOG", "info,johnnyd_bin=debug,johnnyd_lib=debug");
        }
    }
    
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    setup_tracing()?;
    
    tracing::info!("Starting johnnyd");
    tracing::debug!("Debug logging is enabled");
    
    // Example usage of JohnnyD
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let base_dir = PathBuf::from(home_dir).join("johnnyd_test");
    
    let johnny_d = JohnnyD::new(base_dir);
    johnny_d.initialize()?;
    
    tracing::info!("JohnnyD initialized at {:?}", johnny_d.base_dir());
    
    Ok(())
}
