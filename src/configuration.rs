
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::result::{Result};
use std::io::{Error, ErrorKind};


#[derive(Serialize, Deserialize, Debug)]
pub struct ReplacedItem {
    slot: String,
    id: String,
    name: String,
    bonus_id: String,
    gem_id: String,
    relic_id: String,
    enchant_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplacedEnchantment {
    slot: String,
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Replacement {
    items: Vec<ReplacedItem>,
    enchantments: Vec<ReplacedEnchantment>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Simcraft {
    executeable: String,
    html: String,
    txt: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Templates {
    configuration: String,
    character: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration
{
    max_legendaries: i32,
    simcraft: Simcraft,
    templates: Templates,
    replaces: Replacement
}


pub const CONFIG_FILE: &str = "config.json";


impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            max_legendaries: 2,
            simcraft: Simcraft {
                executeable: String::from("simc.exe"),
                html: String::from("output/html/simc_calculation_{}.html"),
                txt: String::from("output/txt/simc_calculation_{}.txt")
            },
            templates: Templates {
                configuration: String::from(""),
                character: String::from("")
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
