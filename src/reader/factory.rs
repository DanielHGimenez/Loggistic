use std::collections::HashMap;
use std::error::Error;
use toml::Value;
use crate::config::parse_options;
use crate::reader::api::Reader;
use crate::reader::simple_file::SimpleFile;

pub fn produce(reader_type: &String, mut options: &mut Option<HashMap<String, Value>>) -> Result<Box<dyn Reader>, Box<dyn Error>> {
    let parsed_options = parse_options(options.get_or_insert(HashMap::new()));
    log::info!("Creating reader: {}.", reader_type);
    match reader_type.as_str() {
        "simple_file" => {
            Ok(Box::new(SimpleFile::from(parsed_options)?))
        }
        r => Err(Box::from(format!("Reader type not existent: {}", r)))
    }
}
