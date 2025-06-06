use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Configuration for the server.
pub struct ServerConfig {
    /// Hostname or IP address where the server will run.
    pub host: String,
    /// Port number for the server.
    pub port: u32,
    /// Timeout for requests in seconds.
    pub request_timeout_seconds: u32,
    /// Maximum payload size in megabytes.
    pub max_payload_size_mb: u32,
    /// Rate limiting configuration (optional).
    pub rate_limiting: Option<RateLimitConfig>,
    /// Logging level for the server.
    pub logging_level: LogLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Configuration for rate limiting.
pub struct RateLimitConfig {
    /// Maximum number of requests allowed per minute.
    pub requests_per_minute: u32,
    /// Burst capacity for rate limiting.
    pub burst: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Logging levels for the server.
pub enum LogLevel {
    /// Debug level logging.
    Debug,
    /// Informational level logging.
    Info,
    /// Warning level logging.
    Warning,
    /// Error level logging.
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}
