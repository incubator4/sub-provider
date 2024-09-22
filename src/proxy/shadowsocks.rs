use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::BaseProxy;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shadowsocks {
    #[serde(flatten)]
    pub base: BaseProxy,
    pub cipher: String,
    pub password: String,
    #[serde(default = "Default::default")]
    pub udp: bool,
    pub plugin: Option<String>,
    pub plugin_opts: Option<HashMap<String, serde_yaml::Value>>,
}
