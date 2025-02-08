use std::collections::HashMap;
use std::error::Error;
use toml::Value;
use crate::config::parse_options;
use crate::provider::api::Provider;
use crate::provider::logstash::LogStash;

pub fn produce(provider_type: &String, options: &mut Option<HashMap<String, Value>>) -> Result<Box<dyn Provider>, Box<dyn Error>> {
    let parsed_options = parse_options(options.get_or_insert(HashMap::new()));
    log::info!("Creating provider: {}.", provider_type);
    match provider_type.as_str() {
        "logstash" => {
            Ok(Box::new(LogStash::new()))
        }
        provider => Err(Box::from(format!("Provider type not existent: {}", provider)))
    }
}
