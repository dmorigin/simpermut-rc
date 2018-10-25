
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate regex;
extern crate clap;
extern crate chrono;
extern crate uuid;
extern crate indicatif;
extern crate rand;

use clap::{Arg, App};
use chrono::Duration;

mod template;

mod simcraft;
use simcraft::*;

mod configuration;
use configuration::*;


const VERSION: &str = "0.3.1";
const AUTHOR: &str = "[DM]Origin";

// This is the time in seconds in which one iteration
// runs. On faster machines its about 12s to 15s. But on
// slower one it could be of around 25s.
// So I set this to 19s. I hope this is a god value :)
const TIME_PER_ITER: u64 = 15;


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
        .arg(Arg::with_name("talents")
            .short("t")
            .long("talents")
            .takes_value(true)
            .help("Override the talent setting from input file."))
        .arg(Arg::with_name("yes")
            .short("y")
            .help("Accept automaticaly the amount of iterations."))
        .get_matches();
    

    // read config file
    let config_file = arg_matches.value_of("config").unwrap_or(CONFIG_FILE);
    let config = configuration::Configuration::load(config_file).unwrap();
    let talents = arg_matches.value_of("talents").unwrap_or("");
    let accept = arg_matches.is_present("yes");

    // Map for all items
    let item_list_file = arg_matches.value_of("INPUT").unwrap();
    println!("Read data from input file: {}", item_list_file);

    // handle simc
    let mut simc = simcraft::Simcraft::new(&config, talents);
    simc.compute_item_list(item_list_file).unwrap();
    
    // calculate the number of iterations
    println!("Calculate the number of iterations...");
    let iterations = simc.calculate_iterations();
    println!("Your request generates absolute {} iterations", iterations.0);
    println!("This runs for approximalty: {}", fmt_duration(iterations.1 * TIME_PER_ITER));
    println!("Do you want to continue? (y == yes / n == no)");

    if accept == false {
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
    } else {
        println!("Automaticaly accept the iterations.");
    }

    // start permutation
    simc.permutation(iterations).unwrap();
}


fn fmt_duration(duration: u64) -> String {
    let duration = Duration::seconds(duration as i64);

    let days = duration.num_days();
    let hours = duration.num_hours() - duration.num_days() * 24;
    let minutes = duration.num_minutes() - duration.num_hours() * 60;
    let seconds = duration.num_seconds() - duration.num_minutes() * 60;

    format!("{} Days - {:0>#2}:{:0>#2}:{:0>#2}", days, hours, minutes, seconds)
}