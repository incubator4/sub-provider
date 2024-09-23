mod common;
mod hysteria2;
pub mod protocol;
mod shadowsocks;
mod socks5;
mod trojan;
mod tuic;
mod vless;
mod vmess;

use crate::error::Error;
use common::BaseProxy;
use serde::{Deserialize, Serialize};

pub const PROXY_DIRECT: &str = "DIRECT";
pub const PROXY_REJECT: &str = "REJECT";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Proxy {
    #[serde(skip)]
    Direct,
    #[serde(skip)]
    Reject,
    #[serde(rename = "ss")]
    Ss(shadowsocks::Shadowsocks),
    #[serde(rename = "socks5")]
    Socks5(socks5::Socks5),
    #[serde(rename = "trojan")]
    Trojan(trojan::Trojan),
    #[serde(rename = "vmess")]
    Vmess(vmess::Vmess),
    #[serde(rename = "vless")]
    Vless(vless::Vless),
    #[serde(rename = "hysteria2")]
    Hysteria2(hysteria2::Hysteria2),
    #[serde(rename = "tuic")]
    Tuic(tuic::Tuic),
}

impl Proxy {
    pub fn name(&self) -> &str {
        match self {
            Proxy::Direct => PROXY_DIRECT,
            Proxy::Reject => PROXY_REJECT,
            Proxy::Ss(ss) => &ss.base.name,
            Proxy::Socks5(socks5) => &socks5.base.name,
            Proxy::Trojan(trojan) => &trojan.base.name,
            Proxy::Vmess(vmess) => &vmess.base.name,
            Proxy::Vless(vless) => &vless.base.name,
            Proxy::Hysteria2(hysteria2) => &hysteria2.base.name,
            Proxy::Tuic(tuic) => &tuic.base.name,
        }
    }
}

impl TryFrom<String> for Proxy {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let u = url::Url::parse(&value).map_err(Error::UrlParse)?;

        let proxy = match u.scheme() {
            "trojan" => trojan::Trojan::try_from(u.clone()).map(Proxy::Trojan),
            "vmess" => vmess::Vmess::try_from(u.clone()).map(Proxy::Vmess),
            "vless" => vless::Vless::try_from(u.clone()).map(Proxy::Vless),
            "hysteria2" => hysteria2::Hysteria2::try_from(u.clone()).map(Proxy::Hysteria2),
            "tuic" => tuic::Tuic::try_from(u.clone()).map(Proxy::Tuic),
            t => Err(Error::ProxyTypeNotSupported(t.to_string())),
        }?;

        Ok(proxy)
    }
}

impl TryInto<String> for Proxy {
    type Error = Error;
    fn try_into(self) -> Result<String, Self::Error> {
        let url: String = match self {
            // Proxy::Trojan(trojan) => trojan.to_url(),
            // Proxy::Vmess(vmess) => vmess.to_url(),
            // Proxy::Vless(vless) => vless.to_url(),
            _ => Err(Error::ProxyTypeNotSupported("".to_string())),
        }?;

        Ok(url)
    }
}
