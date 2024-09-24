use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::{
    common::BaseProxy,
    protocol::{Network, TLS},
};

use crate::util::get_query;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        Ok(Vless {
            base: BaseProxy::try_from(value.clone())?,
            uuid: value.username().to_string(),
            flow: get_query("flow".into(), &value).unwrap_or_default(),
            // packet_encoding: None,
            network: Network::try_from(value.clone()).ok(),

            tls: TLS::try_from(value.clone()).ok(),
        })
    }
}
