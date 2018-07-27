
pub mod slot;
pub mod item_map;
pub mod item;
pub mod report;
pub mod statistic;


use regex::Regex;
use configuration::Configuration;
use std::fs::{File, create_dir_all};
use std::io::{BufRead, BufReader};
use std::result::{Result};
use std::io::{Error};
use std::process::{Command, Stdio};
use uuid::*;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::{Local, Duration};

use item_map::ItemMap;
use item::Item;
use slot::{Slot, ESlot};
use template::Template;
use configuration::{ReplacedEnchantment};
use report::Generator;
use statistic::Statistic;


pub struct Simcraft {
    config: Configuration,
    items: ItemMap,
    template: Template,
    output_dir: String,
    report_dir: String,
    compile_dir: String,
    log_dir: String,
    report: Generator,
    statistic: Statistic,
    spec: String,
    talents: String
}

impl Simcraft {
    pub fn new(config: &Configuration) -> Simcraft {
        // setup directories
        let output_dir = format!("{}/{}", config.output_dir, Uuid::new_v4().to_string());
        let report_dir = format!("{}/{}", output_dir, config.report_dir);
        let compile_dir = format!("{}/compiles", output_dir);
        let log_dir = format!("{}/{}", output_dir, config.log_dir);

        create_dir_all(&report_dir).unwrap();
        create_dir_all(&compile_dir).unwrap();
        create_dir_all(&log_dir).unwrap();

        // add replaced items
        let mut item_map = ItemMap::new();
        for i in &config.replaces.items {
            item_map.push(&Slot::from_str(&i.slot).unwrap(), &Item::from_replaced_item(i));
        }

        // create object
        Simcraft {
            config: (*config).clone(),
            items: item_map,
            template: Template::default(),
            output_dir,
            report_dir: report_dir.clone(),
            compile_dir,
            log_dir,
            report: Generator::new(config, &report_dir),
            statistic: Statistic::new(config),
            spec: String::new(),
            talents: String::new()
        }
    }

    // returns a tuple.
    // .0 => absolut value
    // .1 => approximate value
    pub fn calculate_iterations(&self) -> (u64, u64) {
        let iterations = self._calculate_iterations_at(ESlot::Head, 0) as f64;
        let approximate = 30f32 / self.config.statistic.tolerance; // need to be fixed!

        (iterations as u64, approximate as u64)
    }

    fn _calculate_iterations_at(&self, start_slot: ESlot, skip: usize) -> u64 {
        let mut slot = start_slot;
        let mut iterations = match self.items.get_slot(slot) {
            Some(l) => l.len() - skip,
            None => 1
        };

        // step through all items
        while let Some(s) = Simcraft::next_slot(slot) {
            if s == ESlot::Trinket || s == ESlot::Finger {
                iterations *= match self.items.get_slot(s) {
                    Some(l) => ((l.len() * l.len()) - l.len()) / 2,
                    None => 1
                }
            } else {
                // normal iterations
                iterations *= match self.items.get_slot(s) {
                    Some(l) => l.len(),
                    None => 1
                };
            }

            slot = s;
        }

        iterations as u64
    }

    fn _skipped_iterations(&self, start_slot: ESlot, skip: usize, offset: u64) -> u64 {
        let calculated = self._calculate_iterations_at(start_slot, skip);
        match calculated > offset {
            true => calculated - offset,
            false => 0
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
    pub fn permutation(&mut self, iterations: (u64, u64)) -> Result<(u64), Error> {

        // create stack.
        let mut stack: Vec<Item> = Vec::new();
        let mut progress_bar: ProgressBar = ProgressBar::new(iterations.0 + 2);
        let now = Local::now();

        // setup progress bar
        progress_bar.set_style(
            ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {pos:>7}/{len:7} [{eta_precise}]")
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "));

        println!("Start permutation with approximatly {} iterations", iterations.1);
        println!("You can find the outputs at: {}", self.output_dir);
        println!("Starts at: {}", now.format("%d.%m.%Y - %H:%M:%S"));
        println!("Finished approximatly at: {}",
            (now + Duration::seconds((iterations.1 * ::TIME_PER_ITER) as i64)).format("%d.%m.%Y - %H:%M:%S"));

        // generate template
        let tpl: String = format!("{}/{}", self.config.template_dir, self.config.simcraft.template);
        self.template = Template::load(&tpl).unwrap();

        progress_bar.inc(1);

        // begin permutation
        let iterations = self.permut_iteration_single(
            &mut progress_bar,
            &mut stack,
            ESlot::Head,
            0);

        progress_bar.finish();

        // generate report
        self.report.compile();

        let diff = Local::now() - now;
        println!("Permutation finished: {}", Local::now().format("%d.%m.%Y - %H:%M:%S"));
        println!("after: {}", ::fmt_duration(diff.num_seconds() as u64));

        Ok(iterations.0)
    }

    /// This method process a single iteration of the permutation. This means it will
    /// step through all items of a single slot. For every item of this slot another
    /// permutation method will be called.
    fn permut_iteration_single(&self, 
        progress_bar: &mut ProgressBar,
        stack: &mut Vec<Item>,
        slot: ESlot,
        parse_counter: u64) -> (u64, bool, u64)
    {
        let mut parse_counter = parse_counter;
        let mut iteration_count = 0;

        // off_hand could be empty. So we need to jump over
        let items = match self.items.get_slot(slot) {
            Some(i) => i,
            None => {
                return self.handle_iteration_step(progress_bar, stack, slot, parse_counter);
            }
        };
        
        for item in items {
            // check limits
            if self.has_multiple_of_them(stack) {
                continue;
            }

            iteration_count += 1;

            // skip?
            if self.statistic.ignore(item) {
                progress_bar.inc(self._calculate_iterations_at(slot, iteration_count));
                continue;
            }

            // add my own one
            stack.push(item.clone());

            // step into next iteration
            let tuple = self.handle_iteration_step(progress_bar, stack, slot, parse_counter);
            parse_counter = tuple.0;

            stack.pop(); // remove item from stack

            // We have some items to skip. So we need to check if this item was se last one
            // that is removed from the stack.
            if tuple.1 {
                if self.statistic.has_ignores(stack) {
                    return (parse_counter, true, self._skipped_iterations(slot, iteration_count, tuple.2));
                } else {
                    self.set_progressbar_inc(progress_bar, slot, iteration_count, tuple.2);
                }
            }
        }

        (parse_counter, false, 0)
    }

    /// This method does generaly the same as Simcraft::permut_iteration_single. Except 
    /// that this method steps through all items of a single slot twice. This is needed
    /// because that rings and trinkets has two slots.
    fn permut_iteration_double(&self,
        progress_bar: &mut ProgressBar,
        stack: &mut Vec<Item>,
        slot: ESlot,
        parse_counter: u64) -> (u64, bool, u64)
    {
        let mut parse_counter = parse_counter;
        let mut counter = 0;
        let mut has_ignores = false;
        let mut iteration_count1 = 0;
        let mut skipped = 0u64;

        // slot finger1
        let slot1_items = self.items.get_slot(slot).unwrap();
        for slot1 in slot1_items.iter() {
            // check limits
            if self.has_multiple_of_them(&stack) {
                continue;
            }

            iteration_count1 += 1;

            // skip?
            if self.statistic.ignore(slot1) {
                progress_bar.inc(self._calculate_iterations_at(slot, iteration_count1));
                continue;
            }

            // add my own one
            let mut item = slot1.clone();
            item.slot = Slot::get_real_slot(&slot1.slot, 1).unwrap();
            stack.push(item);

            // slot finger2
            let slot2_items = self.items.get_slot(slot).unwrap();
            for slot2 in slot2_items.iter().skip(counter) {
                // cannot add the same item on both slots
                if slot1.id == slot2.id {
                    continue;
                }

                // check limits
                if self.has_multiple_of_them(&stack) {
                    continue;
                }

                // skip?
                if self.statistic.ignore(slot2) {
                    progress_bar.inc(self._calculate_iterations_at(slot, iteration_count1));
                    continue;
                }

                // add my own one
                let mut item = slot2.clone();
                item.slot = Slot::get_real_slot(&slot2.slot, 2).unwrap();
                stack.push(item);

                // step into next iteration
                let tuple = self.handle_iteration_step(progress_bar, stack, slot, parse_counter);
                parse_counter = tuple.0;
                skipped = tuple.2;

                stack.pop(); // remove item from stack

                if tuple.1 {
                    if self.statistic.has_ignores(stack) {
                        has_ignores = true;
                        break;
                    } else {
                        self.set_progressbar_inc(progress_bar, slot, iteration_count1, tuple.2);
                        has_ignores = false;
                    }
                }
            }

            stack.pop(); // remove item from stack
            counter += 1; // increase the counter to skip items

            if has_ignores {
                if self.statistic.has_ignores(stack) {
                    return (parse_counter, true, self._skipped_iterations(slot, iteration_count1, skipped));
                } else {
                    self.set_progressbar_inc(progress_bar, slot, iteration_count1, skipped);
                    has_ignores = false;
                }
            }
        }

        (parse_counter, false, 0)
    }

    fn build_simc_file(&self, 
        progress_bar: &mut ProgressBar,
        stack: &Vec<Item>,
        parse_counter: u64) -> (u64, bool, u64)
    {
        // setup template
        let parse_counter = parse_counter + 1;

        // build the item list
        let mut item_list: String = String::new();
        for item in stack.iter() {
            let mut entry: String = format!("{}=,id={}", &item.slot.name, item.id);

            if !item.gem_id.is_empty() {
                entry.push_str(&format!(",gem_id={}", item.gem_id));
            }

            if !item.relic_id.is_empty() {
                entry.push_str(&format!(",relic_id={}", item.relic_id));
            }

            if !item.bonus_id.is_empty() {
                entry.push_str(&format!(",bonus_id={}", item.bonus_id));
            }

            if item.enchant_id != 0 {
                entry.push_str(&format!(",enchant_id={}", item.enchant_id));
            }

            item_list.push_str(&entry);
            item_list.push('\n');
        }

        // setup reports
        let report_html = format!("{}/{}", self.report_dir, self.config.simcraft.html.replace("{}", &parse_counter.to_string()));
        let report_json = format!("{}/{}", self.report_dir, self.config.simcraft.json.replace("{}", &parse_counter.to_string()));

        create_dir_all(Simcraft::extract_path(&report_html)).unwrap();
        create_dir_all(Simcraft::extract_path(&report_json)).unwrap();

        self.template.set_var("report_html", &report_html).unwrap();
        self.template.set_var("report_json", &report_json).unwrap();

        // setup list of all items
        self.template.set_var("item_list", &item_list).unwrap();

        if self.template.var_exist("spec") && self.config.simcraft.override_spec {
            self.template.set_var("spec", &self.spec).unwrap();
        }

        if self.template.var_exist("talents") && self.config.simcraft.override_talents {
            self.template.set_var("talents", &self.talents).unwrap();
        }

        // compile template
        let process_tpl = format!("{}/{}", &self.compile_dir,
            self.config.simcraft.process_template.replace("{}", &parse_counter.to_string()));

        //println!("Run {} with compiled template {}", &self.config.simcraft.executeable, &process_tpl);

        Template::store(&process_tpl, &self.template.compile().unwrap()).unwrap();

        // execute template
        let stdout = format!("{}/{}_{}.log", &self.log_dir, "stdout", &parse_counter.to_string());
        let stdout = File::create(&stdout).unwrap();

        let stderr = format!("{}/{}_{}.log", &self.log_dir, "stderr", &parse_counter.to_string());
        let stderr = File::create(&stderr).unwrap();

        let mut process = Command::new(&self.config.simcraft.executeable)
            .arg(process_tpl)
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn().unwrap();
        process.wait().unwrap();

        // generate report
        let tuple = self.report.push(&report_json, &report_html);
        self.statistic.update(stack, tuple.1, tuple.2, tuple.3);

        progress_bar.inc(1);
        (parse_counter, self.statistic.has_ignores(stack), 0)
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
                    let line = line.trim();

                    // read spec from simc
                    let regex_spec = Regex::new("^spec=(.*)$").unwrap();
                    if let Some(spec) = regex_spec.captures(&line) {
                        self.spec = String::from(&spec[1]);
                    }

                    // read talents from simc
                    let regex_talents = Regex::new("^talents=(.*)$").unwrap();
                    if let Some(talents) = regex_talents.captures(&line) {
                        self.talents = String::from(&talents[1]);
                    }

                    let regex_item = Regex::new("(head|neck|shoulder|back|chest|wrist|waist|hands|legs|feet|finger1|finger2|trinket1|trinket2|main_hand|off_hand)=([a-zA-Z0-9]*),(.*)")
                        .unwrap();
                    let regex_ids = Regex::new("(id|gem_id|bonus_id|relic_id|enchant_id)=([\\d/:]+)")
                        .unwrap();

                    // find something
                    for cap_item in regex_item.captures_iter(&line) {
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
                            match &cap_ids[1] {
                                "id" => item.id = String::from(&cap_ids[2]).parse::<u32>().unwrap(),
                                "gem_id" => item.gem_id = String::from(&cap_ids[2]),
                                "bonus_id" => item.bonus_id = String::from(&cap_ids[2]),
                                "relic_id" => item.relic_id = String::from(&cap_ids[2]),
                                "enchant_id" => item.enchant_id = String::from(&cap_ids[2]).parse::<u32>().unwrap(),
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

    fn handle_iteration_step(
        &self,
        progress_bar: &mut ProgressBar,
        stack: &mut Vec<Item>,
        slot: ESlot,
        parse_counter: u64
    ) -> (u64, bool, u64) {
        if let Some(next) = Simcraft::next_slot(slot) {
            let tuple = match next {
                ESlot::Finger => self.permut_iteration_double(progress_bar, stack, next, parse_counter),
                ESlot::Trinket => self.permut_iteration_double(progress_bar, stack, next, parse_counter),
                _ => self.permut_iteration_single(progress_bar, stack, next, parse_counter)
            };

            return tuple;
        }

        self.build_simc_file(progress_bar, stack, parse_counter)
    }

    fn extract_path(path: &str) -> String {
        let p = match String::from(path).rfind('/') {
            Some(n) => n,
            None => {
                return String::from(path);
            }
        };

        String::from(&path[..p])
    }

    fn is_replaced_by_item(&self, slot: &Slot) -> bool {
        for i in &self.config.replaces.items {
            if i.slot == slot.name {
                return true;
            }
        }

        false
    }

    fn get_replaced_enchantment(&self, slot: &Slot) -> Option<ReplacedEnchantment> {
        for i in &self.config.replaces.enchantments {
            if i.slot == slot.name {
                return Some(i.clone());
            }
        }

        None
    }

    fn next_slot(current: ESlot) -> Option<ESlot> {
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

    fn has_multiple_of_them(&self, stack: &Vec<Item>) -> bool {

        // step through all limits
        for i in &self.config.limits {
            let mut count = 0u32;

            // check items
            for s in stack.iter() {
                // is in limit list
                for l in &i.items {
                    if *l == s.id {
                        count += 1;
                    }

                    // limit arrived
                    if count >= i.max {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn set_progressbar_inc(
        &self,
        progress_bar: &mut ProgressBar,
        slot: ESlot,
        iteration_count: usize,
        offset: u64
    ) {
        let mut iterations = self._calculate_iterations_at(slot, iteration_count);
        iterations = match iterations < offset {
            true => 0,
            false => iterations - offset
        };
        progress_bar.inc(iterations);
    }
}
