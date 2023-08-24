# app-properties

This crate provides the `AppPropertiesExt` providing the `load` function with which a configuration file can be loaded.

The proc macro `AppProperties` must be used to implement this trait for your type.

Load your config by calling the `load` function on the type that derives `AppProperties`.

```rust
let properties = AppProperties::load().unwrap() // try to handle the error gracefully!
```

Every type involved requires the `ReplaceEnv` derive macro! The configuration is expected to contain replaceable data.

Let's define the aforementioned exemplar `AppProperties` type.

```rust
use std::fmt::Display;

use app_properties::AppProperties;
use replace_env::ReplaceEnv;
use serde::Deserialize;
use tracing::metadata::LevelFilter;

#[derive(Debug, Deserialize, ReplaceEnv, AppProperties)]
#[app_properties(src = "../../../res/application.yaml")]
pub struct AppProperties {
    #[replace_env(raw_type = "RawApp")]
    pub app: App,
    #[replace_env(raw_type = "RawLogging")]
    pub logging: Logging,
    #[replace_env(raw_type = "RawDatabase")]
    pub database: Database,
    #[replace_env(raw_type = "RawMinio")]
    pub minio: Minio,
    #[replace_env(raw_type = "RawServer")]
    pub server: Server,
    #[replace_env(raw_type = "RawKeycloak")]
    pub keycloak: Keycloak,
    #[replace_env(raw_type = "RawHetzner")]
    pub hetzner: Hetzner,
}

#[derive(Debug, Deserialize, ReplaceEnv)]
pub struct App {
    pub url_prefix: String,
    #[replace_env(raw_type = "String")]
    pub environment: Environment,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Environment {
    #[serde(rename(serialize = "test", deserialize = "test"))]
    Test,
    #[serde(rename(serialize = "prod", deserialize = "prod"))]
    Prod,
}

impl From<String> for Environment {
    fn from(string: String) -> Self {
        match serde_json::from_str(format!("\"{string}\"").as_str()) {
            Ok(ok) => ok,
            Err(err) => panic!("{err:#?}"),
        }
    }
}

#[derive(Debug, Deserialize, ReplaceEnv)]
pub struct Logging {
    #[replace_env(raw_type = "String")]
    pub log_level: LogLevel,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_ansi_coloring: bool,
    pub with_thread_name: bool,
    pub with_thread_id: bool,
}

#[derive(Debug, Deserialize, ReplaceEnv)]
pub struct Database {
    pub address: String,
    pub port: u32,
    pub name: String,
    pub username: String,
    #[replace_env(secret)]
    pub password: String,
    pub query_logging: bool,
}

#[derive(Debug, Deserialize, ReplaceEnv)]
pub struct Minio {
    pub address: String,
    pub port: u32,
    pub username: String,
    #[replace_env(secret)]
    pub password: String,
    pub public_data_bucket: String,
}

#[derive(Debug, Deserialize, ReplaceEnv)]
pub struct Server {
    pub address: String,
    pub port: String,
    #[replace_env(raw_type = "String")]
    pub protocol: Protocol,
    pub tls_cert_path: String,
    pub tls_key_path: String,
}

#[derive(Debug, Deserialize, ReplaceEnv)]
pub struct Keycloak {
    #[replace_env(raw_type = "String")]
    pub protocol: Protocol,
    pub address: String,
    pub port: Option<u32>,
    pub relative_path: Option<String>,
    pub realm: String,
}

#[derive(Debug, Deserialize, ReplaceEnv)]
pub struct Hetzner {
    #[replace_env(secret)]
    pub cloud_api_token: String,
    #[replace_env(secret)]
    pub dns_api_token: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Protocol {
    #[serde(rename(serialize = "http", deserialize = "http"))]
    Http,
    #[serde(rename(serialize = "https", deserialize = "https"))]
    Https,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http => f.write_str("http"),
            Protocol::Https => f.write_str("https"),
        }
    }
}

// TODO: derive automatically
impl From<String> for Protocol {
    fn from(string: String) -> Self {
        match serde_json::from_str(format!("\"{string}\"").as_str()) {
            Ok(ok) => ok,
            Err(err) => panic!("{err:#?}"),
        }
    }
}

/// Available log levels.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize)]
pub enum LogLevel {
    /// No logging is performed.
    #[serde(alias = "off", alias = "Off", alias = "OFF")]
    Off,

    /// Only errors are logged.
    #[serde(alias = "error", alias = "Error", alias = "ERROR")]
    Error,

    /// Warnings and errors are logged.
    #[serde(alias = "warn", alias = "Warn", alias = "WARN")]
    Warn,

    /// Infos, warnings and errors are logged.
    #[serde(alias = "info", alias = "Info", alias = "INFO")]
    Info,

    /// Debug-, info-, warn- and error-logs are logged.
    #[serde(alias = "debug", alias = "Debug", alias = "DEBUG")]
    Debug,

    /// Trace-, debug-, info-, warn- and error-logs are logged.
    #[serde(alias = "trace", alias = "Trace", alias = "TRACE")]
    Trace,
}

impl From<String> for LogLevel {
    fn from(string: String) -> Self {
        match serde_json::from_str(format!("\"{string}\"").as_str()) {
            Ok(ok) => ok,
            Err(err) => panic!("{err:#?}"),
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Off => LevelFilter::OFF,
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}
```

## MSRV

The minimum supported rust version is `1.64.0`
