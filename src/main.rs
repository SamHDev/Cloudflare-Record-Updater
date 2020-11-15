pub mod config;
pub mod service;
pub mod name;
pub mod cloudflare;

use tokio;
use crate::config::Config;
use crate::name::Name;
use crate::cloudflare::{get_zone_id, get_client, get_record_id, get_record, push_record};
use tokio::runtime::Runtime;
use tokio::time::Duration;

#[warn(unused_imports)]
use log::*;


fn main() {
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(handle());
}

async fn handle() {
    // Setup Logging
    env_logger::init();

    println!("Loading {} v{} by {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    let config = Config::read().await;
    let names = Name::from_vec(&config.names).unwrap();
    let parts = Name::separate(&names);
    println!("Loaded Configuration");
    /*
    println!("Domains");
    for part in &parts {
        println!("|   {}", part.0);
        for d in &part.1 {
            println!("|   |   {}", d);
        }
    }
    */

    let client = get_client(&config.api_key);

    println!("\nRecord Map:");
    let mut targets = Vec::new();
    for (hostname, records) in parts {
        let zone_id = get_zone_id(&client, &hostname).await.unwrap();
        println!("{}     =    {}", zone_id, hostname);
        for record in records {
            let record_id = get_record_id(&client, &zone_id, &hostname, &record).await.unwrap();
            println!("| {}   =    | {}", record_id, record);
            targets.push(TargetRecord {
                hostname: hostname.clone(),
                record_name: record.clone(),
                zone_id: zone_id.clone(),
                record_id
            })
        }
    }
    println!();
    let mut ip;

    loop {
        match config.service.get().await {
            None => {println!("Failed to get current IP Address");}
            Some(v) => {ip=v; break;}
        }
        tokio::time::delay_for(Duration::from_secs(config.interval.clone())).await;
    }
    println!("IP address resolved as '{}'", ip);

    for target in &targets {
        match get_record(&client, &target.zone_id, &target.record_id).await {
            Ok(mut record) => {
                if record.ip != ip {
                    //println!("Updating query record for {}/{}", target.record_name, target.hostname);
                    record.ip = ip.clone();
                    if let Err(e) = push_record(&client, &target.zone_id, &target.record_id, &record).await {
                        println!("Failed to update query record for {}/{}: {}", target.record_name, target.hostname, e);
                    } else {
                        println!("Updated query record for {}/{}", target.record_name, target.hostname);
                    }
                }
            }
            Err(_e) => println!("Failed to query record for {}/{}", target.record_name, target.hostname)
        }
    }

    println!("Initial Record update complete");
    println!("\nRunning {} for {} records every {}s", env!("CARGO_PKG_NAME"), targets.len(), config.interval);
    loop {
        let new_ip = match config.service.get().await {
            None => {println!("Failed to get current IP Address"); continue;}
            Some(v) => v
        };
        if new_ip != ip {
            println!("IP address resolved as '{}'", ip);
            for target in &targets {
                match get_record(&client, &target.zone_id, &target.record_id).await {
                    Ok(mut record) => {
                        if record.ip != new_ip {
                            record.ip = new_ip.clone();
                            if let Err(e) = push_record(&client, &target.zone_id, &target.record_id, &record).await {
                                println!("Failed to update query record for {}/{}: {}", target.record_name, target.hostname, e);
                            } else {
                                println!("Updated query record for {}/{}", target.record_name, target.hostname);
                            }
                        }
                    }
                    Err(_e) => println!("Failed to query record for {}/{}", target.record_name, target.hostname)
                }
            }
        }
        ip = new_ip;
        tokio::time::delay_for(Duration::from_secs(config.interval.clone())).await;
    }
}

pub struct TargetRecord {
    hostname: String,
    record_name: String,
    zone_id: String,
    record_id: String
}