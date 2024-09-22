use super::{protocol::Network, BaseProxy};
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Trojan {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_cert_verify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opts: Option<Network>,
}

impl TryFrom<url::Url> for Trojan {
    type Error = Error;
    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let is_grpc = value
            .query_pairs()
            .filter(|(k, _)| k == "type")
            .next()
            .map(|(_, v)| v == "grpc")
            .unwrap_or(false);

        Ok(Trojan {
            base: BaseProxy {
                name: value.host_str().unwrap().to_string(),
                server: value.host_str().unwrap().to_string(),
                port: value.port().unwrap_or(443),
            },
            password: value.username().to_string(),
            alpn: match value.query_pairs().filter(|(k, _)| k == "alpn").next() {
                Some((_, v)) => Some(v.split(',').map(|s| s.to_string()).collect()),
                None => None,
            },
            sni: value
                .query_pairs()
                .filter(|(k, _)| k == "sni")
                .map(|(_, v)| v.to_string())
                .next(),
            skip_cert_verify: match value
                .query_pairs()
                .filter(|(k, _)| k == "skip_cert_verify")
                .next()
            {
                Some((_, v)) => Some(v.parse().unwrap()),
                None => None,
            },
            udp: match value.query_pairs().filter(|(k, _)| k == "udp").next() {
                Some((_, v)) => Some(v.parse().unwrap_or(is_grpc)),
                None => None,
            },
            opts: if is_grpc {
                Some(Network::Grpc {
                    grpc_service_name: value
                        .query_pairs()
                        .filter(|(k, _)| k == "serviceName")
                        .map(|(_, v)| v.to_string())
                        .next(),
                })
            } else {
                None
            },
        })
    }
}
