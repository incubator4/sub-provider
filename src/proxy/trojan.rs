use super::{
    common::BaseProxy,
    protocol::{Network, TLS},
};
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Trojan {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub password: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TLS>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opts: Option<Network>,
}

impl TryFrom<url::Url> for Trojan {
    type Error = Error;
    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        Ok(Trojan {
            base: BaseProxy::try_from(value.clone())?,
            password: value.username().to_string(),
            tls: TLS::try_from(value.clone()).ok(),
            opts: Network::try_from(value.clone()).ok(),
        })
    }
}
