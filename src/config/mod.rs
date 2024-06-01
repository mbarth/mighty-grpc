use config::{Config, ConfigError, File};
use serde::Deserialize;

/// Represents the configuration for a server, either API or gRPC.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    /// The address on which the server will listen.
    pub address: String,
    /// The port on which the server will listen.
    pub port: u16,
}

/// Represents the configuration for the Mighty server.
#[derive(Debug, Deserialize)]
pub struct MightyServerConfig {
    /// The base URL for the Mighty server. This is optional as it's not required when running
    /// in `binary` mode.
    pub base_url: Option<String>,
}

/// Represents the logging configuration.
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    /// The logging level (e.g., "info", "debug").
    pub level: String,
}

/// Represents the entire application settings, which includes gRPC server, API server,
/// Mighty server, and logging configurations.
#[derive(Debug, Deserialize)]
pub struct AppSettings {
    /// Configuration for the gRPC server.
    pub grpc_server: ServerConfig,
    /// Optional configuration for the API server.
    pub api_server: Option<ServerConfig>,
    /// Optional configuration for the Mighty server.
    pub mighty_server: Option<MightyServerConfig>,
    /// Configuration for logging.
    pub logging: LoggingConfig,
}


impl AppSettings {
    /// Loads the application settings from a configuration file named "config.toml".
    ///
    /// This function uses the `config` crate to load the settings from a file and
    /// deserializes them into an `AppSettings` struct.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if the configuration file cannot be read or parsed.
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;
        config.try_deserialize()
    }
}
