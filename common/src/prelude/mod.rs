use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

pub mod error;
pub use error::{Error, Result};

/// Initialize the prelude: error handling and tracing subscriber.
///
/// # Errors
/// Returns an error if the initialization fails, currently only possible if
/// `color_eyre::install` fails to replace the default panic hook.
pub fn prelude() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}
