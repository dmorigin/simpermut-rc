
pub mod slot;
pub mod item_map;
pub mod item;


use regex::Regex;
use configuration::Configuration;
use std::fs::{File, create_dir_all};
use std::io::{BufRead, BufReader};
use std::result::{Result};
use std::io::{Error};
use std::process::{Command, Stdio};
use uuid::*;

use item_map::ItemMap;
use item::Item;
use slot::{Slot, ESlot};
use template::Template;
use configuration::{ReplacedEnchantment};


pub struct Simcraft {
    config: Configuration,
    items: ItemMap,
    template: Template,
    report_dir: String,
    compile_dir: String
}

impl Simcraft {
    pub fn new(config: &Configuration) -> Simcraft {
        // setup directories
        let output_dir = format!("{}/{}", config.output_dir, Uuid::new_v4().to_string());
        let report_dir = format!("{}/{}", output_dir, config.report_dir);
        let compile_dir = format!("{}/compiles", output_dir);

        create_dir_all(&report_dir).unwrap();
        create_dir_all(&compile_dir).unwrap();

        // add replaced items
        let mut item_map = ItemMap::new();
        for i in config.replaces.items.iter() {
            item_map.push(&Slot::from_str(&i.slot).unwrap(), &Item::from_replaced_item(i));
        }

        // create object
        Simcraft {
            config: (*config).clone(),
            items: item_map,
            template: Template::default(),
            report_dir: report_dir,
            compile_dir: compile_dir
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

    /// This function start the whole permutation process. The process starts with
    /// the head slot. This is a single step permutation. Bevor this step, the
    /// configured template will be load.
    pub fn permutation(&mut self) -> Result<bool, Error> {
        // create stack.
        let mut stack: Vec<Item> = Vec::new();

        // generate template
        let tpl: String = format!("{}/{}", self.config.template_dir, self.config.simcraft.template);
        self.template = Template::load("basic", &tpl).unwrap();

        self.permut_iteration_single(&mut stack, &ESlot::Head, 0);
        Ok(true)
    }

    /// This method process a single iteration of the permutation. This means it will
    /// step through all items of a single slot. For every item of this slot another
    /// permutation method will be called.
    fn permut_iteration_single(&self, 
        stack: &mut Vec<Item>,
        slot: &ESlot,
        parse_counter: u64) -> u64
    {
        let mut parse_counter = parse_counter;
        let items = self.items.get_slot(slot).unwrap();
        
        for item in items.iter() {
            // add my own one
            stack.push(item.clone());

            // step into next iteration
            if let Some(next) = Simcraft::next_slot(slot) {
                parse_counter = match next {
                    ESlot::Finger => self.permut_iteration_double(stack, &next, parse_counter),
                    ESlot::Trinket => self.permut_iteration_double(stack, &next, parse_counter),
                    _ => self.permut_iteration_single(stack, &next, parse_counter)
                };
            } else {
                parse_counter = self.build_simc_file(stack, parse_counter);
            }

            stack.pop(); // remove item from stack
        }

        return parse_counter;
    }

    /// This method does generaly the same as Simcraft::permut_iteration_single. Except 
    /// that this method steps through all items of a single slot twice. This is needed
    /// because that rings and trinkets has two slots.
    fn permut_iteration_double(&self,
        stack: &mut Vec<Item>,
        slot: &ESlot,
        parse_counter: u64) -> u64
    {
        let mut parse_counter = parse_counter;
        let mut counter = 0;

        // slot finger1
        let slot1_items = self.items.get_slot(slot).unwrap();
        for slot1 in slot1_items.iter() {
            // add my own one
            let mut item = slot1.clone();
            item.slot = Slot::get_real_slot(&slot1.slot, 1).unwrap();
            stack.push(item);

            // slot finger2
            let slot2_items = self.items.get_slot(slot).unwrap();
            for slot2 in slot2_items.iter().skip(counter) {
                if slot1.id == slot2.id {
                    continue;
                }

                // add my own one
                let mut item = slot2.clone();
                item.slot = Slot::get_real_slot(&slot2.slot, 2).unwrap();
                stack.push(item);

                // step into next iteration
                if let Some(next) = Simcraft::next_slot(slot) {
                    parse_counter = match next {
                        ESlot::Finger => self.permut_iteration_double(stack, &next, parse_counter),
                        ESlot::Trinket => self.permut_iteration_double(stack, &next, parse_counter),
                        _ => self.permut_iteration_single(stack, &next, parse_counter)
                    };
                } else {
                    parse_counter = self.build_simc_file(stack, parse_counter);
                }

                stack.pop(); // remove item from stack
            }

            stack.pop(); // remove item from stack
            counter += 1; // increase the counter to skip items
        }

        return parse_counter;
    }

    fn build_simc_file(&self, 
        stack: &Vec<Item>, 
        parse_counter: u64) -> u64
    {
        // build the item list
        let mut item_list: String = String::new();
        for item in stack.iter() {
            let mut entry: String = format!("{}=,id={}", &item.slot.name, item.id);

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

        // setup template
        let parse_counter = parse_counter + 1;

        // setup reports
        let report_html = format!("{}/{}", self.report_dir, self.config.simcraft.html.replace("{}", &format!("{}", parse_counter.to_string())));
        let report_json = format!("{}/{}", self.report_dir, self.config.simcraft.json.replace("{}", &format!("{}", parse_counter.to_string())));

        create_dir_all(Simcraft::extract_path(&report_html)).unwrap();
        create_dir_all(Simcraft::extract_path(&report_json)).unwrap();

        self.template.set_var("report_html", &report_html).unwrap();
        self.template.set_var("report_json", &report_json).unwrap();

        // setup list of all items
        self.template.set_var("item_list", &item_list).unwrap();

        // compile template
        let process_tpl = format!("{}/{}", &self.compile_dir,
            self.config.simcraft.process_template.replace("{}", &parse_counter.to_string()));

        println!("Run {} with compiled template {}", &self.config.simcraft.executeable, &process_tpl);

        Template::store(&process_tpl, &self.template.compile().unwrap()).unwrap();

        // execute template
        let stdout = format!("{}/{}", &self.compile_dir, "stdout.log");
        let stdout = File::create(&stdout).unwrap();

        let stderr = format!("{}/{}", &self.compile_dir, "stderr.log");
        let stderr = File::create(&stderr).unwrap();
/*
        let mut process = Command::new(&self.config.simcraft.executeable)
            .arg(process_tpl)
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn().unwrap();
        process.wait().unwrap();
        */
        return parse_counter;
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
                        //println!("items: {:?}", cap_item);

                        // save slot
                        let slot = match Slot::from_str(&cap_item[1]) {
                            Ok(slot) => slot,
                            Err(_) => continue
                        };

                        // is replaced slot
                        if self.is_replaced_by_item(&slot) {
                            continue;
                        }

                        let mut item: Item = Item::new();

                        // has replaced enchantment
                        if let Some(enchant) = self.get_replaced_enchantment(&slot) {
                            item.enchant_id = enchant.id;
                        }

                        // save name
                        item.name = String::from(&cap_item[2]);
                        item.slot = slot.clone();

                        // extract id's
                        for cap_ids in regex_ids.captures_iter(&cap_item[3]) {
                            //println!("ids: {:?}", cap_ids);

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

    fn extract_path(path: &str) -> String {
        let p = match String::from(path).rfind("/") {
            Some(n) => n,
            None => {
                return String::from(path);
            }
        };

        String::from(&path[..p])
    }

    fn is_replaced_by_item(&self, slot: &Slot) -> bool {
        for i in self.config.replaces.items.iter() {
            if i.slot == slot.name {
                return true;
            }
        }

        false
    }

    fn get_replaced_enchantment(&self, slot: &Slot) -> Option<ReplacedEnchantment> {
        for i in self.config.replaces.enchantments.iter() {
            if i.slot == slot.name {
                return Some(i.clone());
            }
        }

        None
    }

    fn next_slot(current: &ESlot) -> Option<ESlot> {
        match current {
            ESlot::Head => Some(ESlot::Neck),
            ESlot::Neck => Some(ESlot::Shoulder),
            ESlot::Shoulder => Some(ESlot::Back),
            ESlot::Back => Some(ESlot::Chest),
            ESlot::Chest => Some(ESlot::Waist),
            ESlot::Waist => Some(ESlot::Wrist),
            ESlot::Wrist => Some(ESlot::Hands),
            ESlot::Hands => Some(ESlot::Feet),
            ESlot::Feet => Some(ESlot::Legs),
            ESlot::Legs => Some(ESlot::Finger),
            ESlot::Finger => Some(ESlot::Trinket),
            ESlot::Trinket => Some(ESlot::MainHand),
            ESlot::MainHand => Some(ESlot::OffHand),
            _ => None
        }
    }
}