use reqwest;
use serde::{Serialize, Deserialize};
use serde_json as json;

pub fn get_client(api_key: &str) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json")
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", &api_key)).unwrap()
    );
    reqwest::Client::builder().default_headers(headers).build().unwrap()
}

pub async fn get_zone_id(client: &reqwest::Client, hostname: &str) -> Result<String, String> {
    match client.get("https://api.cloudflare.com/client/v4/zones")
        .query(&[("name", hostname), ("status", "active")])
        .send().await {
        Ok(query) => match query.text().await {
            Ok(text) => match json::from_str::<CloudflareResponse<Vec<CloudflareIdInstance>>>(&text) {
                Ok(data) => match data.reswrap()?.get(0) {
                    Some(id) => Ok(id.id.clone()),
                    None => Err(format!("No results for zone query"))
                },
                Err(e) => Err(format!("Failed to parse response json: {}\n{}", e.to_string(), &text))
            },
            Err(e) => Err(format!("Failed to read response text: {}", e.to_string()))
        }
        Err(_e) => Err("Failed to query cloudflare API".to_string())
    }
}
/*match query.json::<CloudflareResponse<CloudflareIdInstance>>().await {
            Ok(data) => Ok(data.reswrap()?.id),
            Err(e) => Err(format!("Failed to parse response json: {:?}", e))
        },*/

pub async fn get_record_id(client: &reqwest::Client, zone_id: &str, hostname: &str, record: &str) -> Result<String, String> {
    match client.get(&format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", zone_id))
        .query(&[("name",
            if hostname == record {
                hostname.to_string()
            } else {
                format!("{}.{}", record, hostname)
            }
        )])
        .send().await {
        Ok(query) => match query.text().await {
            Ok(text) => match json::from_str::<CloudflareResponse<Vec<CloudflareIdInstance>>>(&text) {
                Ok(data) => match data.reswrap()?.get(0) {
                    Some(id) => Ok(id.id.clone()),
                    None => Err(format!("No results for zone query"))
                },
                Err(e) => Err(format!("Failed to parse response json: {}\n{}", e.to_string(), &text))
            },
            Err(e) => Err(format!("Failed to read response text: {}", e.to_string()))
        }
        Err(_e) => Err("Failed to query cloudflare API".to_string())
    }
}

pub async fn get_record(client: &reqwest::Client, zone_id: &str, record_id: &str) -> Result<CloudflareRecordInstance, String> {
    match client.get(&format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, record_id)).send().await {
        Ok(query) => match query.text().await {
            Ok(text) => match json::from_str::<CloudflareResponse<CloudflareRecordInstance>>(&text) {
                Ok(data) => data.reswrap(),
                Err(e) => Err(format!("Failed to parse response json: {}\n{}", e.to_string(), &text))
            },
            Err(e) => Err(format!("Failed to read response text: {}", e.to_string()))
        }
        Err(_e) => Err("Failed to query cloudflare API".to_string())
    }
}

pub async fn push_record(client: &reqwest::Client, zone_id: &str, record_id: &str, record: &CloudflareRecordInstance) -> Result<(), String> {
    match client.post(&format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, record_id))
        .json(record)
        .send()
        .await {
        Ok(a) => match a.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write: {}", e.to_string()))
        },
        Err(_e) => Err("Failed to query cloudflare API".to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CloudflareResponse<T> {
    pub success: bool,
    pub errors: Vec<CloudflareError>,
    pub result: Option<T>
}

impl<T> CloudflareResponse<T> {
    pub fn reswrap(self) -> Result<T, String> {
        Ok(self.result()?.unwrap())
    }
    pub fn result(self) -> Result<Option<T>, String> {
        if self.success {
            Ok(self.result)
        } else {
            match self.errors.get(0) {
                Some(e) =>  Err(format!("{} ({})", e.message, e.code)),
                None => Err("Unknown Error (-1)".to_string())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CloudflareError {
    pub code: u16,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct CloudflareIdInstance {
    pub id: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloudflareRecordInstance {
    pub id: String,
    #[serde(rename="type")]
    pub record_type: String,
    #[serde(rename="content")]
    pub ip: String,
    pub ttl: u32,
    pub proxied: bool
}
