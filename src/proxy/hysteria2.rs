use super::{protocol::TLS, BaseProxy};
use crate::{error::Error, util::get_query};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hysteria2 {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub password: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ports: String,
    // pub up: String,
    // pub down: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub obfs: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub obfs_password: String,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TLS>,
}

impl TryFrom<url::Url> for Hysteria2 {
    type Error = Error;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        Ok(Hysteria2 {
            base: BaseProxy::try_from(value.clone())?,
            password: value.username().to_string(),
            ports: get_query("ports", &value).unwrap_or_default(),
            obfs: get_query("obfs", &value).unwrap_or_default(),
            obfs_password: get_query("obfs-password", &value).unwrap_or_default(),

            tls: TLS::try_from(value).ok(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url() {
        let url = "hysteria2://password@hostname:1123?peer=hostname&insecure=0&sni=hostname&alpn=h3#hysteria2-proxy";

        let hysteria2 = Hysteria2::try_from(url::Url::parse(url).unwrap()).unwrap();

        assert_eq!(hysteria2.base.name, "hysteria2-proxy");
        assert_eq!(hysteria2.base.server, "hostname");
        assert_eq!(hysteria2.base.port, 1123);
        assert_eq!(hysteria2.base.ip_version, None);
        assert_eq!(hysteria2.base.udp, false);

        assert_eq!(hysteria2.password, "password");
        assert_eq!(hysteria2.ports, "");
        assert_eq!(hysteria2.obfs, "");
        assert_eq!(hysteria2.obfs_password, "");
    }
}
