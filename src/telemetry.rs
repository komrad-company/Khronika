use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::configuration::{TelemetryConfiguration, TelemetryOutput};

pub fn intialize_logger(config: TelemetryConfiguration) {
    let filter = EnvFilter::new(config.level.as_str());
    let registry = tracing_subscriber::registry().with(filter);

    let (file, _) = match config.output {
        TelemetryOutput::File { file } => (Some(file), None),
        TelemetryOutput::Remote { telemetry } => (None, Some(telemetry)),
        TelemetryOutput::Both { file, telemetry } => (Some(file), Some(telemetry)),
    };

    let stderr_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(std::io::stderr);

    let file_layer = file.map(|path| {
        let appender =
            tracing_appender::rolling::never(path.parent().unwrap(), path.file_name().unwrap());

        tracing_subscriber::fmt::layer()
            .json()
            .with_writer(appender)
    });

    // let remote = telemetry_url.map(|url| {});

    registry.with(stderr_layer).with(file_layer).init();
}
