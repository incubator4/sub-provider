use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::BaseProxy;

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

// impl TryFrom<url::Url> for Shadowsocks {
//     type Error = anyhow::Error;

//     fn try_from(url: url::Url) -> Result<Self, Self::Error> {
//         let base = BaseProxy::try_from(url)?;

//         let mut cipher = None;
//         let mut password = None;
//         let mut udp = None;
//         let mut plugin = None;
//         let mut plugin_opts = None;

//         for (key, value) in url.query_pairs() {
//             match key.as_ref() {
//                 "cipher" => cipher = Some(value.into_owned()),
//                 "password" => password = Some(value.into_owned()),
//                 "udp" => udp = Some(value.parse()?),
//                 "plugin" => plugin = Some(value.into_owned()),
//                 "plugin_opts" => {
//                     plugin_opts = Some(serde_urlencoded::from_str(&value)?);
//                 }
//                 _ => {}
//             }
//         }

//         Ok(Shadowsocks {
//             base,
//             cipher: cipher.ok_or_else(|| anyhow::anyhow!("missing cipher"))?,
//             password: password.ok_or_else(|| anyhow::anyhow!("missing password"))?,
//             udp: udp.unwrap_or(false),
//             plugin,
//             plugin_opts,
//         })
//     }
// }
