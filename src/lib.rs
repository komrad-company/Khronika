#![forbid(unsafe_code)]

pub use tracing::{debug, error, info, trace, warn};

pub(crate) mod configuration;
pub(crate) mod errors;
pub(crate) mod telemetry;

pub use configuration::{TelemetryConfiguration, TelemetryOutput};
pub use errors::Error;
pub use opentelemetry_sdk::logs::SdkLoggerProvider;
pub use telemetry::initialize_logger;
