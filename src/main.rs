
#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate clap;
extern crate chrono;
extern crate singleton;

use std::fs::File;
use clap::{Arg, App};

mod template;
use template::*;

mod simcraft;
use simcraft::*;

mod configuration;
use configuration::*;


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
    let config_file = arg_matches.value_of("config").unwrap_or(CONFIG_FILE);
    let config = configuration::Configuration::load(config_file).unwrap();


    // Map for all items
    let item_list_file = arg_matches.value_of("INPUT").unwrap();
    println!("Read data from input file: {}", item_list_file);

    // handle simc
    let simc = simcraft::Simcraft::new(&config);
    simc.compute_item_list(item_list_file).unwrap();
    simc.permutation();
}

