use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::UNIX_EPOCH;
use serde_derive::Deserialize;
use toml::Value;

#[derive(Deserialize)]
pub struct Provider {
    pub name: String,
    pub r#type: String,
    pub options: Option<HashMap<String, Value>>
}

#[derive(Deserialize)]
pub struct Reader {
    pub name: String,
    pub r#type: String,
    pub providers: Vec<String>,
    pub options: Option<HashMap<String, Value>>
}

#[derive(Deserialize)]
pub struct Config {
    pub providers: Vec<Provider>,
    pub readers: Vec<Reader>,
}

pub fn get_last_modified_date(file_name: &str) -> Result<u64, Box<dyn Error>> {
    Ok(fs::metadata(file_name)?.modified()?.duration_since(UNIX_EPOCH)?.as_secs())
}

pub fn load(file_name: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(file_name)?;
    match toml::from_str::<Config>(&contents) {
        Ok(config) => Ok(config),
        Err(e) => Err(Box::new(e))
    }
}

pub fn parse_options(options: &HashMap<String, Value>) -> HashMap<String, String> {
    log::info!("Parsing options.");
    options.iter()
        .map(|(key, value)| {
            return match value {
                Value::String(string) => (key.clone(), string.clone()),
                _ => (key.clone(), value.to_string().clone())
            }
        })
        .collect::<HashMap<String, String>>()
}
