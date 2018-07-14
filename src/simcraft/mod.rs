
pub mod slot;
pub mod item_map;
pub mod item;


use regex::Regex;
use configuration::Configuration;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::result::{Result};
use std::io::{Error, ErrorKind};

use item_map::ItemMap;
use item::Item;
use slot::Slot;
use template::Template;


pub struct Simcraft {
    config: Configuration,
    items: ItemMap,
    template: Template,
    parse_counter: u64
}

impl Simcraft {
    pub fn new(config: &Configuration) -> Simcraft {
        Simcraft {
            config: (*config).clone(),
            items: ItemMap::new(),
            template: Template::default(),
            parse_counter: 0u64
        }
    }

    pub fn compute_item_list(&mut self, file: &str) -> Result<bool, Error> {
        match File::open(file) {
            Ok(file) => {
                self.parse_simc_file(&file);
                println!("Number of keys: {}", self.items.len());
                Ok(true)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn permutation(&mut self) -> Result<bool, Error> {
        // create stack.
        let mut stack: Vec<Item> = Vec::new();

        // generate template
        self.template = Template::load("basic", &self.config.template).unwrap();

        self.permut_iterations(&mut stack, 0);
        Ok(true)
    }


    fn permut_iterations(&mut self, stack: &mut Vec<Item>, count: usize)
    {
        // End of array arrived
        if count >= self.items.len() {
            // build up simc file
            self.build_simc_file(stack);
            return;
        }

        let item_pair = &self.items[count];
        
        for iter in item_pair.items.iter() {
            // add my own one
            stack.push(iter.clone());

            // step into next iteration
            self.permut_iterations(stack, count + 1);

            // remove from stack
            stack.pop();
        }
    }

    fn build_simc_file(&mut self, stack: &Vec<Item>)
    {
        // build the item list
        let mut item_list: String = String::new();
        for item in stack.iter() {
            let mut entry: String = format!("{}=,id={}", item.slot.name(), item.id);

            if item.gem_id.len() > 0 {
                entry.push_str(&format!(",gem_id={}", item.gem_id));
            }

            if item.relic_id.len() > 0 {
                entry.push_str(&format!(",relic_id={}", item.relic_id));
            }

            if item.bonus_id.len() > 0 {
                entry.push_str(&format!(",bonus_id={}", item.bonus_id));
            }

            if item.enchant_id.len() > 0 {
                entry.push_str(&format!(",enchant_id={}", item.enchant_id));
            }

            item_list.push_str(&entry);
            item_list.push('\n');
        }

        // setup config template
        self.parse_counter = self.parse_counter + 1u64;

        let output_html = self.config.simcraft.html.replace("{}", &self.parse_counter.to_string());
        let output_txt = self.config.simcraft.txt.replace("{}", &self.parse_counter.to_string());

        self.template.set_var("output_html", &output_html);
        self.template.set_var("output_txt", &output_txt);

        // setup character template
    }

    /// Search for item declarations
    /// 
    /// The method search inside a *.simc file for item declarations.
    /// A simc file is provided by the SimulationCraft application. You
    /// can find it here: http://simulationcraft.org/
    /// 
    /// The file that you want use here stores all items in your bag and/or
    /// bank. The best way to generate this *.simc file is to use an
    /// addon. One of these addon is called simulationcraft
    /// https://www.curseforge.com/wow/addons/simulationcraft
    /// 
    /// Install it, type /simc in the chat box and copy all the text into
    /// a file. Now save it to something.simc.
    fn parse_simc_file(&mut self, stream : &File)
    {
        // read all in a buffer
        let buffer = BufReader::new(stream);

        // step through alle lines
        for (_num, line) in buffer.lines().enumerate() {
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
                        let slot = match Slot::from_str(&cap_item[1]) {
                            Ok(slot) => slot,
                            Err(_) => continue
                        };

                        let mut item: Item = Item::new();

                        // save name
                        item.name = String::from(&cap_item[2]);
                        item.slot = slot;

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
                        self.items.push(&slot, &item);
                    }
                },
                // end of file 
                Err(_) => break
            }
        }
    }
}
