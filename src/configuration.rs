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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_output_deserializes_to_file_variant() {
        let config: TelemetryConfiguration =
            serde_json::from_str(r#"{"level":"info","file":"output/app.log"}"#).unwrap();
        assert!(matches!(config.output, TelemetryOutput::File { .. }));
    }

    #[test]
    fn remote_output_deserializes_to_remote_variant() {
        let config: TelemetryConfiguration =
            serde_json::from_str(r#"{"level":"warn","telemetry":"https://otlp.example.com"}"#)
                .unwrap();
        assert!(matches!(config.output, TelemetryOutput::Remote { .. }));
    }

    #[test]
    fn both_outputs_deserialize_to_both_variant() {
        let config: TelemetryConfiguration = serde_json::from_str(
            r#"{"level":"debug","file":"output/app.log","telemetry":"https://otlp.example.com"}"#,
        )
        .unwrap();
        assert!(matches!(config.output, TelemetryOutput::Both { .. }));
    }

    #[test]
    fn valid_level_parsed_correctly() {
        let config: TelemetryConfiguration =
            serde_json::from_str(r#"{"level":"error","file":"output/app.log"}"#).unwrap();
        assert_eq!(config.level, tracing::Level::ERROR);
    }

    #[test]
    fn invalid_level_returns_deserialization_error() {
        assert!(
            serde_json::from_str::<TelemetryConfiguration>(
                r#"{"level":"verbose","file":"output/app.log"}"#
            )
            .is_err()
        );
    }

    #[test]
    fn missing_output_returns_deserialization_error() {
        assert!(serde_json::from_str::<TelemetryConfiguration>(r#"{"level":"info"}"#).is_err());
    }
}
