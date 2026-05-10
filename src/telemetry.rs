use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, WithExportConfig};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::configuration::{TelemetryConfiguration, TelemetryOutput};
use crate::errors::Error;

pub fn intialize_logger(
    config: TelemetryConfiguration,
) -> Result<Option<SdkLoggerProvider>, Error> {
    let filter = EnvFilter::new(config.level.as_str());
    let registry = tracing_subscriber::registry().with(filter);

    let (file_path, telemetry_url) = match config.output {
        TelemetryOutput::File { file } => (Some(file), None),
        TelemetryOutput::Remote { telemetry } => (None, Some(telemetry)),
        TelemetryOutput::Both { file, telemetry } => (Some(file), Some(telemetry)),
    };

    let stderr_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(std::io::stderr);

    let file_layer = file_path.map(|path| {
        let appender =
            tracing_appender::rolling::never(path.parent().unwrap(), path.file_name().unwrap());

        tracing_subscriber::fmt::layer()
            .json()
            .with_writer(appender)
    });

    let logger_provider = telemetry_url
        .map(|url| -> Result<SdkLoggerProvider, Error> {
            let exporter = LogExporter::builder()
                .with_http()
                .with_endpoint(url)
                .build()?;

            Ok(SdkLoggerProvider::builder()
                .with_batch_exporter(exporter)
                .build())
        })
        .transpose()?;

    let remote_layer = logger_provider
        .as_ref()
        .map(OpenTelemetryTracingBridge::new);

    registry
        .with(stderr_layer)
        .with(file_layer)
        .with(remote_layer)
        .init();

    Ok(logger_provider)
}
