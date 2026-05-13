use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to build OTLP log exporter: {0}")]
    OtlpExporterBuild(#[from] opentelemetry_sdk::logs::LogError),
    #[error("invalid log file path: {0}")]
    InvalidFilePath(PathBuf),
}
