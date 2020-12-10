use serde::{Serialize, Deserialize};
use crate::service::ResolverService::IPIFY;
use reqwest::get;

#[derive(Serialize, Deserialize, Debug)]
pub enum ResolverService {
    #[serde(rename="ipify")]
    IPIFY
}

impl Default for ResolverService {
    fn default() -> Self {
        ResolverService::IPIFY
    }
}

impl ResolverService {
    pub async fn get(&self) -> Option<String> {
        match &self {
            IPIFY => {
                match get("https://api.ipify.org/?format=json").await {
                    Ok(r) => Some(r.json::<IpifyResponse>().await.ok()?.ip),
                    Err(_e) => None
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpifyResponse {
    pub ip: String
}