use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::proxy::Proxy;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Clash {
    port: u16,
    socks_port: u16,
    allow_lan: bool,
    mode: String,
    log_level: String,
    external_controller: String,
    secret: String,
    proxies: Vec<Proxy>,
    proxy_groups: Vec<ProxyGroup>,
    rules: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ProxyGroup {
    name: String,
    #[serde(rename = "type")]
    group_type: String,
    proxies: Vec<String>,
    url: Option<String>,
    interval: Option<u64>,
}

impl Clash {
    pub fn new() -> Self {
        Self {
            port: 7890,
            socks_port: 7891,
            allow_lan: false,
            mode: "Rule".to_string(),
            log_level: "info".to_string(),
            external_controller: "".to_string(),
            secret: "".to_string(),
            proxies: vec![],
            proxy_groups: vec![],
            rules: vec![],
        }
    }

    pub fn with_proxies(mut self, proxies: HashMap<String, Vec<Proxy>>) -> Self {
        self.proxies = proxies.values().flatten().cloned().collect();
        self.proxy_groups = proxies
            .iter()
            .map(|(name, p)| ProxyGroup {
                name: name.clone(),
                group_type: "select".to_string(),
                proxies: p.iter().map(|p| p.name().to_string()).collect(),
                url: None,
                interval: None,
            })
            .collect();
        self
    }
}

impl super::Provider for Clash {
    fn provide(&self) -> String {
        serde_yaml::to_string(self).unwrap_or_default()
    }
}
