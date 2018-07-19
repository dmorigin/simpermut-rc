
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate regex;
extern crate clap;
extern crate chrono;
extern crate uuid;
extern crate indicatif;

use clap::{Arg, App};
use chrono::Duration;

mod template;

mod simcraft;
use simcraft::*;

mod configuration;
use configuration::*;


const VERSION: &str = "0.1.2";
const AUTHOR: &str = "[DM]Origin";

// This is the time in seconds in which one iteration
// runs. On faster machines its about 12s to 15s. But on
// slower one it could be of around 25s.
// So I set this to 19s. I hope this is a god value :)
const TIME_PER_ITER: u64 = 19;


fn main() {
    // read application arguments
    let arg_matches = App::new("SimPermut")
        .version(VERSION)
        .author(AUTHOR)
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
        .arg(Arg::with_name("version")
            .long("version")
            .help("Print the version of this application."))
        .get_matches();
    

    // read config file
    let config_file = arg_matches.value_of("config").unwrap_or(CONFIG_FILE);
    let config = configuration::Configuration::load(config_file).unwrap();


    // Map for all items
    let item_list_file = arg_matches.value_of("INPUT").unwrap();
    println!("Read data from input file: {}", item_list_file);

    // handle simc
    let mut simc = simcraft::Simcraft::new(&config);
    simc.compute_item_list(item_list_file).unwrap();
    
    // calculate the number of iterations
    let iterations = simc.permutation(true, 0).unwrap();
    println!("Your request generates {} iterations", iterations);
    println!("This runs for approximalty: {}", fmt_duration(iterations * TIME_PER_ITER));
    println!("Do you want to continue? (y == yes / n == no)");

    loop {
        let mut accept: String = String::new();
        std::io::stdin().read_line(&mut accept)
            .expect("Failed to read user input");
        
        match &accept.trim()[..] {
            "y" => { break; },
            "Y" => { break; },
            "n" => { return; },
            "N" => { return; }
            _ => ()
        }
    }

    // start permutation
    simc.permutation(false, iterations).unwrap();
}


fn fmt_duration(duration: u64) -> String {
    let duration = Duration::seconds(duration as i64);

    let days = duration.num_days();
    let hours = duration.num_hours() - duration.num_days() * 24;
    let minutes = duration.num_minutes() - duration.num_hours() * 60;
    let seconds = duration.num_seconds() - duration.num_minutes() * 60;

    format!("{} Days - {:0>#2}:{:0>#2}:{:0>#2}", days, hours, minutes, seconds)
}