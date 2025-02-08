mod provider;
mod reader;
mod config;
mod boot;

use crate::boot::configure_logs;
use crate::config::{get_last_modified_date, load, Config};
use crate::provider::{
    api::Provider,
    factory::produce as produce_provider
};
use crate::reader::{
    api::Reader,
    factory::produce as produce_reader
};
use nix::unistd::sleep;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    configure_logs();
    log::info!("Starting...");
    let config_file = "config.toml";
    let mut last_time_modified = get_last_modified_date(config_file).unwrap();
    let mut providers: HashMap<String, Box<dyn Provider>> = HashMap::new();
    let mut readers: HashMap<String, Box<dyn Reader>> = HashMap::new();

    loop {
        if get_last_modified_date(config_file).unwrap() > last_time_modified
            || providers.is_empty()
            || readers.is_empty()
        {
            log::info!("New configurations detected.");
            last_time_modified = get_last_modified_date(config_file).unwrap();
            let config: Config = load(config_file).expect("Failed to load config.toml");

            log::info!("Cleaning old environment.");
            for mut reader in readers.values_mut() {
                reader.stop();
            }
            for mut provider in providers.values_mut() {
                provider.stop();
            }
            readers.clear();
            providers.clear();

            log::info!("Loading providers.");
            for mut provider_config in config.providers {
                let provider = produce_provider(&provider_config.r#type, &mut provider_config.options).unwrap();
                providers.insert(provider_config.name, provider);
            }

            log::info!("Loading readers.");
            for mut reader_config in config.readers {
                let mut reader = produce_reader(&reader_config.r#type, &mut reader_config.options).unwrap();
                for provider_name in reader_config.providers {
                    match providers.get_mut(&provider_name) {
                        Some(provider) => {
                            provider.add_broadcast(reader.broadcast());
                        }
                        None => {}
                    }
                }
                reader.start();
                readers.insert(reader_config.name, reader);
            }

            for mut provider in providers.values_mut() {
                provider.start();
            }
        }

        sleep(5);
    }
}
