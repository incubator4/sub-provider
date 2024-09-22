pub mod protocol;
mod shadowsocks;
mod socks5;
mod trojan;
mod vless;
mod vmess;

use serde::{Deserialize, Serialize};

use crate::{config::RawProxy, error::Error};

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
        }
    }

    pub fn with_name(&self, name: &str) -> Self {
        match self {
            Proxy::Direct => Proxy::Direct,
            Proxy::Reject => Proxy::Reject,
            Proxy::Ss(ss) => Proxy::Ss(shadowsocks::Shadowsocks {
                base: BaseProxy {
                    name: name.to_string(),
                    ..ss.base.clone()
                },
                ..ss.clone()
            }),
            Proxy::Socks5(socks5) => Proxy::Socks5(socks5::Socks5 {
                base: BaseProxy {
                    name: name.to_string(),
                    ..socks5.base.clone()
                },
                ..socks5.clone()
            }),
            Proxy::Trojan(trojan) => Proxy::Trojan(trojan::Trojan {
                base: BaseProxy {
                    name: name.to_string(),
                    ..trojan.base.clone()
                },
                ..trojan.clone()
            }),
            Proxy::Vmess(vmess) => Proxy::Vmess(vmess::Vmess {
                base: BaseProxy {
                    name: name.to_string(),
                    ..vmess.base.clone()
                },
                ..vmess.clone()
            }),
            Proxy::Vless(vless) => Proxy::Vless(vless::Vless {
                base: BaseProxy {
                    name: name.to_string(),
                    ..vless.base.clone()
                },
                ..vless.clone()
            }),
        }
    }
    pub fn from_raw(raw: RawProxy) -> Result<Self, Error> {
        Proxy::try_from((&raw).clone())
    }
}

impl TryFrom<RawProxy> for Proxy {
    type Error = Error;
    fn try_from(value: RawProxy) -> Result<Self, Self::Error> {
        let u = url::Url::parse(&value.url).map_err(Error::UrlParse)?;
        let name = String::from(value.name.as_str());

        let proxy = match u.scheme() {
            "trojan" => trojan::Trojan::try_from(u.clone()).map(Proxy::Trojan),
            "vmess" => vmess::Vmess::try_from(u.clone()).map(Proxy::Vmess),
            "vless" => vless::Vless::try_from(u.clone()).map(Proxy::Vless),
            t => Err(Error::ProxyTypeNotSupported(t.to_string())),
        }?;

        Ok(proxy.with_name(&name))
    }
}

pub fn from_raw_proxies(raw_proxies: Vec<RawProxy>) -> Vec<Proxy> {
    raw_proxies
        .iter()
        .filter_map(|raw| Proxy::from_raw(raw.clone()).ok())
        .collect()
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct BaseProxy {
    pub name: String,
    pub server: String,
    pub port: u16,
}
