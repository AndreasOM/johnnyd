use color_eyre::eyre::Result;
use johnnyd_lib::JohnnyD;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

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

    // Use the current working directory
    let current_dir = std::env::current_dir()?;
    tracing::info!("Using current directory: {:?}", current_dir);

    // Create a new JohnnyD instance
    let johnny_d = JohnnyD::new(&current_dir);

    // Report if this is the first run
    if johnny_d.first_run() {
        tracing::info!("First run detected, will create default configuration");
    } else {
        tracing::info!("Using existing configuration");
    }

    // Initialize the Johnny Decimal structure
    johnny_d.initialize()?;

    tracing::info!("JohnnyD initialized at {:?}", johnny_d.base_dir());

    Ok(())
}
