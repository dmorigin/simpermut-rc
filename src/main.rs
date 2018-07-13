
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate regex;
extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use clap::{Arg, App};


#[derive(PartialEq, PartialOrd, Eq, Hash)]
enum Slot
{
    Head,
    Neck,
    Shoulder,
    Back,
    Chest,
    Wrist,
    Hands,
    Waist,
    Legs,
    Feet,
    Finger1,
    Finger2,
    Trinket1,
    Trinket2,
    MainHand,
    OffHand
}


#[derive(Default)]
struct Item
{
    id: String,
    name: String,
    bonus_id: String,
    gem_id: String,
    relic_id: String,
    enchant_id: String,
}


struct ItemMap {
    slot: Slot,
    items: Vec<Item>,
}

impl PartialEq for ItemMap {
    fn eq(&self, other: &ItemMap) -> bool {
        self.slot == other.slot
    }
}

impl Eq for ItemMap {
}


#[derive(Serialize, Deserialize, Debug)]
struct ReplacedItem {
    slot: String,
    id: String,
    name: String,
    bonus_id: String,
    gem_id: String,
    relic_id: String,
    enchant_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReplacedEnchantment {
    slot: String,
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Replacement {
    items: Vec<ReplacedItem>,
    enchantments: Vec<ReplacedEnchantment>
}

#[derive(Serialize, Deserialize, Debug)]
struct Simcraft {
    executeable: String,
    html: String,
    txt: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Templates {
    configuration: String,
    character: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration
{
    version: String,
    max_legendaries: i32,
    simcraft: Simcraft,
    templates: Templates,
    replaces: Replacement
}


const CONFIG_FILE: &str = "config.json";


fn main() {
    // read application arguments
    let arg_matches = App::new("SimPermut")
        .version("0.0.1")
        .author("[DM]Origin")
        .about("Generate a permutation of all items set by a simc file.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Set an alternativ configuration file.")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Set the simc file that stores all items that you want to permut.")
            .required(true)
            .index(1))
        .get_matches();
    

    // read config file
    let mut config = Configuration {
        version: String::from("0.0.1"),
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
    };

    let config_file = arg_matches.value_of("config").unwrap_or(CONFIG_FILE);
    match File::open(config_file) {
        Ok(json) => {
            config = serde_json::from_reader(json).unwrap();
        },
        Err(err) => {
            println!("Failed to open config file: {:?}", err);
            return;
        }
    }

    println!("configuration {:?}", config); // debug only


    // Map for all items
    let item_list_file = arg_matches.value_of("INPUT").unwrap();
    let mut items: Vec<ItemMap> = Vec::new();

    println!("Read data from input file: {}", item_list_file);

    // read item list. This is a normal simc file. Generated from an addon
    // inside wow or written by hand.
    match File::open(item_list_file) {
        Ok(file) => {
            parse_simc_file(&file, &mut items);
            println!("Number of keys: {}", items.len());
            permutation(&config, &items);
        }
        Err(err) => {
            println!("Cannot open file {:?}", err);
            return;
        }
    }
}


fn permutation(config: &Configuration, items: &Vec<ItemMap>) {
    let mut collection: Vec<String> = Vec::new();

    permut_iterations(&config.replaces, items, &mut collection, 0);
}


fn permut_iterations(
    replacement: &Replacement,
    items: &Vec<ItemMap>,
    collection: &mut Vec<String>,
    count: usize)
{
    // End of array arrived
    if count >= items.len() {
        return;
    }

    let item_map = &items[count];
    
    match replacement.items.iter().position(|&x| x == item_map.slot) {
        Ok(_) => true,
        Err(_) => false
    }

    for iter in item_map.items.iter() {
        // if count is zero, this is the first entrie. So we need to clear the collection
        if count == 0 {
            collection.clear();
        }

        // add my own one

        // step into next iteration
        permut_iterations(replacement, items, collection, count + 1);
    }
}


fn parse_simc_file(stream : &File, items: &mut Vec<ItemMap>)
{
    // read all in a buffer
    let buffer = BufReader::new(stream);

    // step through alle lines
    for (num, line) in buffer.lines().enumerate() {
        // search for something like this
        // [head|shoulder|...]=[string],id=123,...
        match line {
            Ok(line) => {
                let regex_item = Regex::new("(head|neck|shoulder|back|chest|wrist|waist|hands|legs|feet|finger1|finger2|trinket1|trinket2|main_hand|off_hand)=([a-zA-Z0-9]*),(.*)")
                    .unwrap();
                let regex_ids = Regex::new("(id|gem_id|bonus_id|relic_id|enchant_id)=([\\d/:]+)")
                    .unwrap();
                
                // find something
                for cap_item in regex_item.captures_iter(&line.trim()) {
                    println!("items: {:?}", cap_item);

                    // save slot
                    let slot = match slot(&cap_item[1]) {
                        Ok(slot) => slot,
                        Err(_) => continue
                    };

                    let mut item: Item = Item::default();

                    // save name
                    item.name = String::from(&cap_item[2]);

                    // extract id's
                    for cap_ids in regex_ids.captures_iter(&cap_item[3]) {
                        println!("ids: {:?}", cap_ids);

                        match &cap_ids[1] {
                            "id" => item.id = String::from(&cap_ids[2]),
                            "gem_id" => item.gem_id = String::from(&cap_ids[2]),
                            "bonus_id" => item.bonus_id = String::from(&cap_ids[2]),
                            "relic_id" => item.relic_id = String::from(&cap_ids[2]),
                            "enchant_id" => item.enchant_id = String::from(&cap_ids[2]),
                            _ => ()
                        }
                    }

                    // store new item
                    match item_map_index(items, &slot) {
                        Some(at) => items[at].items.push(item),
                        None => {
                            let mut v: Vec<Item> = Vec::new();
                            v.push(item);
                            items.push(ItemMap {slot, items: v});
                        }
                    }
                }
            },
            // end of file 
            Err(_) => break
        }
    }
}


// work around... there is no "find" method?
fn item_map_index(items: &Vec<ItemMap>, slot: &Slot) -> Option<usize> {
    let mut i: usize = 0;

    for iter in items.iter() {
        if iter.slot == *slot {
            return Some(i);
        }

        i = i + 1;
    }

    return None;
}


fn slot(name: &str) -> std::result::Result<Slot, std::io::Error> {
    match name {
        "head" => Ok(Slot::Head),
        "neck" => Ok(Slot::Neck),
        "shoulder" => Ok(Slot::Shoulder),
        "back" => Ok(Slot::Back),
        "chest" => Ok(Slot::Chest),
        "wrist" => Ok(Slot::Wrist),
        "waist" => Ok(Slot::Waist),
        "hands" => Ok(Slot::Hands),
        "feet" => Ok(Slot::Feet),
        "legs" => Ok(Slot::Legs),
        "finger1" => Ok(Slot::Finger1),
        "finger2" => Ok(Slot::Finger2),
        "trinket1" => Ok(Slot::Trinket1),
        "trinket2" => Ok(Slot::Trinket2),
        "main_hand" => Ok(Slot::MainHand),
        "off_hand" => Ok(Slot::OffHand),
        _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, String::from("Invalid slot name")))
    }
}