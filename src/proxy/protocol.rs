use std::collections::HashMap;

use crate::util::is_false;
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TLS {
    #[serde(skip_serializing_if = "is_false")]
    pub tls: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alpn: Vec<String>,
    #[serde(rename = "servername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub skip_cert_verify: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality_opts: Option<RealityOpts>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RealityOpts {
    pub public_key: String,
    pub short_id: String,
}

#[derive(Debug, Clone)]
pub enum Network {
    Http {
        method: String,
        path: Vec<String>,
        headers: HashMap<String, Vec<String>>,
    },
    H2 {
        host: Vec<String>,
        path: String,
    },
    Grpc {
        grpc_service_name: Option<String>,
    },
    Ws {
        path: String,
        headers: HashMap<String, String>,
        max_early_data: Option<usize>,
    },
}

impl Serialize for Network {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Network::Http {
                method,
                path,
                headers,
            } => {
                let mut state = serializer.serialize_struct("Network", 2)?;
                state.serialize_field("network", "http")?;
                state.serialize_field(
                    "http-opts",
                    &serde_json::json!({
                        "method": method,
                        "path": path,
                        "headers": headers
                    }),
                )?;
                state.end()
            }
            Network::H2 { host, path } => {
                let mut state = serializer.serialize_struct("Network", 2)?;
                state.serialize_field("network", "h2")?;
                state.serialize_field(
                    "h2-opts",
                    &serde_json::json!({
                        "host": host,
                        "path": path
                    }),
                )?;
                state.end()
            }
            Network::Grpc { grpc_service_name } => {
                let mut state = serializer.serialize_struct("Network", 1)?;
                state.serialize_field("network", "grpc")?;
                state.serialize_field(
                    "grpc-opts",
                    &serde_json::json!({
                        "grpc_service_name": grpc_service_name
                    }),
                )?;
                state.end()
            }
            Network::Ws {
                path,
                headers,
                max_early_data,
            } => {
                let mut state = serializer.serialize_struct("Network", 3)?;
                state.serialize_field("network", "ws")?;
                let json_data = match max_early_data {
                    Some(v) => {
                        serde_json::json!({
                            "path": path,
                            "headers": headers,
                            "max_early_data": v
                        })
                    }
                    None => {
                        serde_json::json!({
                            "path": path,
                            "headers": headers
                        })
                    }
                };
                state.serialize_field("ws-opts", &serde_json::json!(json_data))?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Implement custom deserialization logic here
        // This is a simplified example and may require more error handling
        let map: serde_json::Value = Deserialize::deserialize(deserializer)?;

        match map.get("network").and_then(|v| v.as_str()) {
            Some("http") => {
                let opts = map
                    .get("http-opts")
                    .ok_or_else(|| serde::de::Error::missing_field("http-opts"))?;
                let method = opts
                    .get("method")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::missing_field("method"))?
                    .to_string();
                let path = opts
                    .get("path")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| serde::de::Error::missing_field("path"))?
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();

                let headers = opts
                    .get("headers")
                    .and_then(|v| v.as_object())
                    .unwrap_or(&serde_json::Map::new())
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            v.as_array()
                                .unwrap()
                                .iter()
                                .filter_map(|s| s.as_str())
                                .map(|s| s.to_string())
                                .collect(),
                        )
                    })
                    .collect();

                Ok(Network::Http {
                    method,
                    path,
                    headers,
                })
            }
            Some("h2") => {
                let opts = map
                    .get("h2-opts")
                    .ok_or_else(|| serde::de::Error::missing_field("h2-opts"))?;
                let host = opts
                    .get("host")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| serde::de::Error::missing_field("host"))?
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();

                let path = opts
                    .get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::missing_field("path"))?
                    .to_string();

                Ok(Network::H2 { host, path })
            }
            Some("grpc") => {
                let opts = map
                    .get("grpc-opts")
                    .ok_or_else(|| serde::de::Error::missing_field("grpc-opts"))?;
                let grpc_service_name = opts
                    .get("grpc_service_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                Ok(Network::Grpc { grpc_service_name })
            }
            Some("ws") => {
                let opts = map
                    .get("ws-opts")
                    .ok_or_else(|| serde::de::Error::missing_field("ws-opts"))?;
                let path = opts
                    .get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::missing_field("path"))?
                    .to_string();

                let headers = opts
                    .get("headers")
                    .and_then(|v| v.as_object())
                    .unwrap_or(&serde_json::Map::new())
                    .iter()
                    .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
                    .collect();

                let max_early_data = opts
                    .get("max_early_data")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);

                Ok(Network::Ws {
                    path,
                    headers,
                    max_early_data,
                })
            }
            _ => Err(serde::de::Error::custom(format!("unknown network type"))),
        }
    }
}
