mod lib;

use lib::*;
use std::{env, process};

fn main() {
    // collect arguments into vec
    let args: Vec<String> = env::args().collect();
    // parse them into Arguments struct or exit if there is an error
    let arguments: Arguments = Arguments::get(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}\n\nUSAGE:\n    food [lower_bound] [upper_bound] [daily_meals] [total_days]", err);
        process::exit(1);
    });

    // exit if def_nutrients fails
    //if let Err(e) = def_nutrients() {
    //    eprintln!("Application error: {}. Is the food.db present?", e);
    //    process::exit(1);
    //}

    // get nutrient_vec from db
    //let nutrient_vec = def_nutrients().unwrap();

    //println!("{:?}", match_bounds(nutrient_vec, arguments));
    def_nutrients()
}
