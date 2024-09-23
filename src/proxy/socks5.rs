use serde::{Deserialize, Serialize};

use super::common::BaseProxy;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Socks5 {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default = "Default::default")]
    pub tls: bool,
    pub sni: Option<String>,
    #[serde(default = "Default::default")]
    pub skip_cert_verify: bool,
    #[serde(default = "Default::default")]
    pub udp: bool,
}
