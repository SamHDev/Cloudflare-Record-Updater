use crate::service::ResolverService;
use tokio::fs;
use tokio::io::{ErrorKind};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub api_key: String,
    pub service: ResolverService,
    pub names: Vec<String>,
    pub interval: u64
}
impl Default for Config {
    fn default() -> Self {
        Self {
            api_key:  "ENTER_API_KEY_HERE".to_string(),
            service: Default::default(),
            names: vec![],
            interval: 300
        }
    }
}

pub const CONFIG_PATH: &str = "Config.Toml";

impl Config {
    pub async fn read() -> Config {
        match fs::read_to_string(&CONFIG_PATH).await {
            Ok(data) => match toml::from_str(&data) {
                Ok(d) => d,
                Err(e) => {
                    println!("Failed to parse config: - {}", e.to_string());
                    std::process::exit(1);
                }
            },
            Err(e) => if e.kind() == ErrorKind::NotFound {
                fs::write(
                    &CONFIG_PATH,
                    toml::to_string_pretty(&Config::default()).unwrap()
                ).await.expect("Failed to write config");
                println!("Created default config at \"{}\". Please edit before continuing", &CONFIG_PATH);
                std::process::exit(1);
            } else {
                println!("Failed to read config: {:?}", e.kind());
                std::process::exit(1);
            }
        }
    }
}