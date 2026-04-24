use std::path::PathBuf;

use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TelemetryOutput {
    Both { file: PathBuf, telemetry: String },
    File { file: PathBuf },
    Remote { telemetry: String },
}

#[derive(Deserialize)]
pub struct TelemetryConfiguration {
    #[serde(deserialize_with = "deserialize_level")]
    pub level: tracing::Level,
    #[serde(flatten)]
    pub output: TelemetryOutput,
}

fn deserialize_level<'de, D>(deserializer: D) -> Result<tracing::Level, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<tracing::Level>()
        .map_err(serde::de::Error::custom)
}
