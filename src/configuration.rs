
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::result::{Result};
use std::io::{Error, ErrorKind};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplacedItem {
    pub slot: String,
    pub id: String,
    pub name: String,
    pub bonus_id: String,
    pub gem_id: String,
    pub relic_id: String,
    pub enchant_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub executeable: String,
    pub html: String,
    pub txt: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration
{
    pub max_legendaries: i32,
    pub template: String,
    pub simcraft: Simcraft,
    pub replaces: Replacement
}


pub const CONFIG_FILE: &str = "config.json";


impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            max_legendaries: 2,
            template: String::from(""),
            simcraft: Simcraft {
                executeable: String::from("simc.exe"),
                html: String::from("output/html/simc_calculation_{}.html"),
                txt: String::from("output/txt/simc_calculation_{}.txt")
            },
            replaces: Replacement {
                items: Vec::new() as Vec<ReplacedItem>,
                enchantments: Vec::new() as Vec<ReplacedEnchantment>
            }
        }
    }

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
