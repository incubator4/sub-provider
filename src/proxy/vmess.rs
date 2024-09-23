use core::str;

use super::{common::BaseProxy, protocol::Network};
use crate::error::Error;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vmess {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub uuid: String,
    #[serde(rename = "alterId")]
    pub alter_id: u16,
    pub cipher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_cert_verify: Option<bool>,
    #[serde(rename = "servername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub network: Option<Network>,
}

impl TryFrom<url::Url> for Vmess {
    type Error = Error;

    // vmess use base64 encode
    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let byte_data = STANDARD
            .decode(value.host().unwrap().to_string().as_bytes())
            .unwrap();
        let str_data = str::from_utf8(&byte_data).unwrap();
        let value: Value = serde_json::from_str(str_data).unwrap();

        Ok(Vmess {
            base: BaseProxy {
                name: value["ps"].as_str().unwrap().to_string(),
                server: value["host"].as_str().unwrap().to_string(),
                port: value["port"].as_u64().unwrap() as usize,
                ip_version: None,
                udp: false,
            },
            uuid: value["id"].as_str().unwrap().to_string(),
            alter_id: value["aid"].as_u64().unwrap() as u16,
            cipher: Some("auto".into()),
            udp: None,
            tls: match value["tls"].as_str() {
                Some("tls") => Some(true),
                _ => None,
            },
            skip_cert_verify: None,
            server_name: None,
            network: match value["net"].as_str() {
                Some("ws") => Some(Network::Ws {
                    path: value["path"]
                        .as_str()
                        .map(|s| s.to_string())
                        .unwrap_or("".into()),
                    headers: {
                        let mut headers = std::collections::HashMap::new();
                        if let Some(host) = value["host"].as_str() {
                            headers.insert("Host".into(), host.into());
                        }
                        headers
                    }
                    .into_iter()
                    .collect(),
                    max_early_data: None,
                }),
                _ => None,
            },
        })
    }
}
