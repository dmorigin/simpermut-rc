
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::result::{Result};
use std::io::{Error, ErrorKind};


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplacedItem {
    pub slot: String,
    pub id: String,
    pub name: String,
    pub bonus_id: String,
    pub gem_id: String,
    pub relic_id: String,
    pub enchant_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplacedEnchantment {
    pub slot: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replacement {
    pub items: Vec<ReplacedItem>,
    pub enchantments: Vec<ReplacedEnchantment>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Simcraft {
    pub template: String,
    pub process_template: String,
    pub executeable: String,
    pub html: String,
    pub json: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration
{
    pub max_legendaries: i32,
    pub output_dir: String,
    pub report_dir: String,
    pub template_dir: String,
    pub simcraft: Simcraft,
    pub replaces: Replacement
}


pub const CONFIG_FILE: &str = "config.json";


impl Configuration {
    pub fn load(file: &str) -> Result<Configuration, Error> {
        match File::open(file) {
            Ok(json) => {
                let config: Configuration = match serde_json::from_reader(json) {
                    Ok(r) => r,
                    Err(err) => {
                        return Err(Error::new(ErrorKind::InvalidData,
                            format!("Cannot read json data: {}", err)));
                    }
                };

                Ok(config)
            },
            Err(err) => {
                Err(Error::new(ErrorKind::NotFound,
                    format!("Failed to open config file: {:?}", err)))
            }
        }
    }
}
