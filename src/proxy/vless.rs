use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::{
    protocol::{Network, RealityOpts, TLS},
    BaseProxy,
};

use crate::util::get_query;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vless {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub uuid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub flow: String,
    // pub packet_encoding: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TLS>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}

impl TryFrom<url::Url> for Vless {
    type Error = Error;
    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let security = get_query("security", &value).map(String::from);

        let net = get_query("type", &value).map(String::from);

        Ok(Vless {
            base: BaseProxy {
                name: value.host_str().unwrap().to_string(),
                server: value.host_str().unwrap().to_string(),
                port: value.port().unwrap_or(443),
            },

            uuid: value.username().to_string(),
            flow: get_query("flow".into(), &value).unwrap_or_default(),
            // packet_encoding: None,
            network: match net.as_ref().map(|s| s.as_str()) {
                Some("ws") => Some(Network::Ws {
                    path: value
                        .query_pairs()
                        .filter(|(k, _)| k == "path")
                        .next()
                        .map(|(_, v)| v.to_string())
                        .unwrap_or("/".to_string()),
                    headers: {
                        let mut headers = std::collections::HashMap::new();
                        match get_query("sni", &value).map(String::from) {
                            Some(sni) => {
                                headers.insert("Host".to_string(), sni);
                            }
                            None => {
                                headers.insert(
                                    "Host".to_string(),
                                    value.host_str().unwrap().to_string(),
                                );
                            }
                        }
                        headers
                    },
                    max_early_data: None,
                }),
                Some("grpc") => Some(Network::Grpc {
                    grpc_service_name: value
                        .query_pairs()
                        .filter(|(k, _)| k == "serviceName")
                        .map(|(_, v)| v.to_string())
                        .next(),
                }),
                _ => None,
            },

            tls: match security.as_ref().map(|s| s.as_str()) {
                Some("tls") => Some(TLS {
                    server_name: value
                        .query_pairs()
                        .filter(|(k, _)| k == "sni")
                        .map(|(_, v)| v.to_string())
                        .next(),
                    skip_cert_verify: false,
                    tls: true,
                    alpn: match value.query_pairs().filter(|(k, _)| k == "alpn").next() {
                        Some((_, v)) => v.split(',').map(|s| s.to_string()).collect(),
                        None => vec![],
                    },
                    reality_opts: None,
                }),
                Some("reality") => Some(TLS {
                    server_name: value
                        .query_pairs()
                        .filter(|(k, _)| k == "sni")
                        .map(|(_, v)| v.to_string())
                        .next(),
                    skip_cert_verify: true,
                    tls: true,
                    alpn: match value.query_pairs().filter(|(k, _)| k == "alpn").next() {
                        Some((_, v)) => v.split(',').map(|s| s.to_string()).collect(),
                        None => vec![],
                    },
                    reality_opts: Some(RealityOpts {
                        public_key: value
                            .query_pairs()
                            .filter(|(k, _)| k == "pbk")
                            .map(|(_, v)| v.to_string())
                            .next()
                            .unwrap_or_default(),
                        short_id: value
                            .query_pairs()
                            .filter(|(k, _)| k == "sid")
                            .map(|(_, v)| v.to_string())
                            .next()
                            .unwrap_or_default(),
                    }),
                }),
                _ => None,
            },
        })
    }
}
