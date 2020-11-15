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
                match get("https://api.ipify.org/").await {
                    Ok(r) => r.text().await.ok(),
                    Err(_e) => None
                }
            }
        }
    }
}