use std::str::FromStr;

use crate::util::is_false;
use crate::{error::Error, util::get_query};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct BaseProxy {
    pub name: String,
    pub server: String,
    pub port: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<IpVersion>,
    #[serde(skip_serializing_if = "is_false")]
    pub udp: bool,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum IpVersion {
    Ipv4,
    Ipv6,
    Ipv4Prefer,
    Ipv6Prefer,
    Dual,
}

impl TryFrom<url::Url> for BaseProxy {
    type Error = Error;

    fn try_from(url: url::Url) -> Result<Self, Self::Error> {
        let ip_version = get_query("ip_version", &url).map(|s| IpVersion::from_str(&s).unwrap());

        let udp = get_query("udp", &url)
            .map(|s| s.parse().unwrap())
            .unwrap_or(false);

        Ok(BaseProxy {
            name: url.fragment().unwrap_or_default().to_string(),
            server: url.host_str().unwrap().to_string(),
            port: url.port().map(|p| p as usize).unwrap_or(443),
            ip_version,
            udp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url() {
        let p = BaseProxy {
            name: "test".to_string(),
            server: "test.com".to_string(),
            port: 443,
            ip_version: Some(IpVersion::Ipv4),
            udp: false,
        };
        let url = url::Url::parse("https://test.com:443?ip_version=ipv4#test").unwrap();

        assert_eq!(BaseProxy::try_from(url).unwrap(), p);
    }
}
