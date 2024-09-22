use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Toml error '{0}'")]
    Toml(#[from] toml::de::Error),

    #[error("IO error '{0}'")]
    Io(#[from] std::io::Error),

    #[error("Url parse error '{0}'")]
    UrlParse(#[from] url::ParseError),

    #[error("Proxy type not supported '{0}'")]
    ProxyTypeNotSupported(String),
}
