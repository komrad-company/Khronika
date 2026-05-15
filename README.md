# Khronika

![CI](https://github.com/komrad-company/Khronika/actions/workflows/ci.yml/badge.svg) ![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue) ![Rust 2024](https://img.shields.io/badge/rust-edition%202024-orange?logo=rust)

> *"A collective that cannot observe itself is blind. Blindness is not tolerated."*
> — Komrad Engineering Collective, 2026

Khronika is the unified logging and telemetry layer of the Komrad ecosystem. It initialises the log pipeline once — stderr, file, remote endpoint, or any combination — and re-exports the standard `tracing` macros for use across all crates. Every crate in the ecosystem that emits logs does so through Khronika. There is no other approved channel.

```
your-crate
    └── khronika::intialize_logger(config)
             ├── stderr  (JSON, always)
             ├── file    (JSON, optional)
             └── remote  (OTLP/HTTP, optional)
```

---

## API

The collective exposes two things. Two is sufficient.

```rust
use khronika::{intialize_logger, error, warn, info, debug, trace};
use khronika::configuration::TelemetryConfiguration;

fn main() {
    let config: TelemetryConfiguration = /* deserialised from JSON */;
    let _provider = intialize_logger(config).expect("logger initialisation failed");

    info!("The collective is watching.");
}
```

### Public types

| Type | Role |
|---|---|
| `TelemetryConfiguration` | Logger configuration — level and output target(s) |
| `TelemetryOutput` | Output target: `File`, `Remote`, or `Both` |
| `SdkLoggerProvider` | OpenTelemetry provider — keep alive for the process lifetime |
| `Error` | Fatal errors — the caller must handle them or face consequences |

### `intialize_logger`

```rust
pub fn intialize_logger(config: TelemetryConfiguration) -> Result<Option<SdkLoggerProvider>, Error>
```

Initialises the tracing registry with the configured outputs. Returns an `SdkLoggerProvider` when a remote endpoint is configured — the caller must keep it alive for the entire process lifetime or remote log flushing stops.

Always emits to stderr in JSON. File and remote outputs are additive.

### Configuration format

`TelemetryConfiguration` is deserialised from JSON:

```json
{ "level": "info", "file": "output/app.log" }

{ "level": "warn", "telemetry": "https://otlp.example.com" }

{ "level": "debug", "file": "output/app.log", "telemetry": "https://otlp.example.com" }
```

| Field | Type | Required | Description |
|---|---|---|---|
| `level` | `string` | ✅ | Minimum log level: `error`, `warn`, `info`, `debug`, `trace` |
| `file` | `string` | — | Path to the local log file |
| `telemetry` | `string` | — | OTLP/HTTP endpoint for remote log forwarding |

At least one of `file` or `telemetry` must be present.

### Re-exported macros

Khronika re-exports the `tracing` macros directly. Use these everywhere — `println!` is forbidden outside tests.

```rust
use khronika::{error, warn, info, debug, trace};
```

| Macro | Semantic |
|---|---|
| `error!` | Non-recoverable failure |
| `warn!` | Recoverable anomaly |
| `info!` | Business event |
| `debug!` | Normal execution flow |
| `trace!` | Verbose detail |

---

## Usage

Add to `Cargo.toml`:

```toml
khronika = { git = "https://github.com/komrad-company/Khronika.git", tag = "v1.x.x" }
```

Production use requires a git tag. `path` dependencies are permitted in local development only, via `[patch]`.

---

## Dependencies

Each dependency was evaluated by the collective before admission. None were added lightly.

| Crate | Purpose |
|---|---|
| `tracing` | Structured log macros |
| `tracing-subscriber` | Registry, filters, JSON formatting |
| `tracing-appender` | Non-blocking file writer |
| `opentelemetry_sdk` | OpenTelemetry log pipeline |
| `opentelemetry-otlp` | OTLP/HTTP exporter |
| `opentelemetry-appender-tracing` | Bridge between tracing and OpenTelemetry |
| `serde` + `serde_json` | Configuration deserialisation |
| `thiserror` | Error type derivation |

---

## License

AGPL-3.0-or-later — the source remains open, as all things should be.
