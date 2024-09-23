use super::BaseProxy;
use crate::{error::Error, util::get_query};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::str::FromStr;
#[derive(Serialize, Deserialize, Debug, Clone)]
// Tuic V5
pub struct Tuic {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub password: String,
    pub uuid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heartbeat_interval: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    alpn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    udp_relay_mode: Option<RelayMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    congestion_controller: Option<CongestionController>,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RelayMode {
    Native,
    Quic,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CongestionController {
    Cubic,
    NewReno,
    Bbr,
}

impl TryFrom<url::Url> for Tuic {
    type Error = Error;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        Ok(Tuic {
            base: BaseProxy::try_from(value.clone())?,
            password: value.password().unwrap_or_default().to_string(),
            uuid: value.username().to_string(),
            ip: get_query("ip", &value),
            heartbeat_interval: get_query("heartbeat_interval", &value)
                .map(|s| s.parse::<usize>().unwrap()),
            alpn: value
                .query_pairs()
                .filter(|(k, _)| k == "alpn")
                .map(|(_, v)| v.split(',').map(|s| s.to_string()).collect())
                .next()
                .unwrap_or_default(),
            udp_relay_mode: get_query("udp_relay_mode", &value)
                .map(|m| RelayMode::from_str(&m))
                .and_then(|m| m.ok()),
            congestion_controller: get_query("congestion_control", &value)
                .map(|c| CongestionController::from_str(&c))
                .and_then(|c| c.ok()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url() {
        let url = "tuic://some-uuid:other-password@hostname:1123?sni=hostname&congestion_control=bbr&udp_relay_mode=quic&alpn=h3#tuic-proxy";

        let tuic = Tuic::try_from(url::Url::parse(url).unwrap()).unwrap();

        assert_eq!(tuic.base.name, "tuic-proxy");
        assert_eq!(tuic.base.server, "hostname");
        assert_eq!(tuic.base.port, 1123);

        assert_eq!(tuic.uuid, "some-uuid");
        assert_eq!(tuic.password, "other-password");

        assert_eq!(tuic.udp_relay_mode, Some(RelayMode::Quic));
        assert_eq!(tuic.congestion_controller, Some(CongestionController::Bbr));
    }
}
