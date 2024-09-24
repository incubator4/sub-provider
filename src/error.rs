use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Toml error '{0}'")]
    Toml(#[from] toml::de::Error),

    #[error("IO error '{0}'")]
    Io(#[from] std::io::Error),

    #[error("Url parse error '{0}'")]
    UrlParse(#[from] url::ParseError),

    #[error("Invalid network '{0}'")]
    InvalidNetwork(String),

    #[error("Invalid tls")]
    InvalidTLS,

    #[error("Invalid relay mode '{0}'")]
    InvalidRelayMode(String),

    #[error("Invalid congestion controller '{0}'")]
    CongestionController(String),

    #[error("Proxy type not supported '{0}'")]
    ProxyTypeNotSupported(String),
}
