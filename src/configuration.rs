
use std::fs::File;
use std::result::{Result};
use std::io::{Error, ErrorKind};
use serde_json::from_reader as read_config;


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Limit {
    pub max: u32,
    pub items: Vec<u32>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplacedItem {
    pub slot: String,
    pub id: u32,
    pub name: String,
    pub bonus_id: String,
    pub gem_id: String,
    pub relic_id: String,
    pub enchant_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplacedEnchantment {
    pub slot: String,
    pub id: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replacement {
    pub items: Vec<ReplacedItem>,
    pub enchantments: Vec<ReplacedEnchantment>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Simcraft {
    pub best_of: usize,
    pub template: String,
    pub process_template: String,
    pub executeable: String,
    pub html: String,
    pub json: String,
    pub override_spec: bool,
    pub override_talents: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Statistic {
    pub tolerance: f32,
    pub threshold: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration
{
    pub output_dir: String,
    pub report_dir: String,
    pub template_dir: String,
    pub log_dir: String,
    pub simcraft: Simcraft,
    pub replaces: Replacement,
    pub limits: Vec<Limit>,
    pub statistic: Statistic
}


pub const CONFIG_FILE: &str = "config.json";


impl Configuration {
    pub fn load(file: &str) -> Result<Configuration, Error> {
        match File::open(file) {
            Ok(json) => {
                let config: Configuration = match read_config(json) {
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


/*
 * Legendaries for Demon Hunter
137061 // Raddons Kaskadenblick (265)
132444 // Prydaz, Xavarics Magnum Opus (265)
144279 // Größenwahn (265)
137071 // Schulterstücke des Runenmeisters (265)
151798 // Chaostheorie (265)
137066 // Umhang der Teufelsflammen (265)
137014 // Achor, der ewige Hunger (265)
137090 // Bionische Stabilisatoren der Mo'arg (265)
137091 // Verlorene Unterarmschienen des Entweihers (265)
144292 // Geist der Finsterflamme (265)
133976 // Cinidaria der Symbiont (265)
151799 // Umarmung des Vergessens (265)
138949 // Kirel narak (265)
137022 // Das Opfer von Loramus Thalipedes (265)
151639 // Seele des Rächers (265)
152626 // Insigne der Großen Armee (265)
138854 // Fragment des Kerkers des Verräters (265)
144249 // Archimondes wiedergeborener Hass (265)
144259 // Kil'jaedens brennender Wunsch (265)
*/