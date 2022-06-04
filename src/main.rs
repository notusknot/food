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

    // get nutrient_vec from db
    let nutrient_vec = match def_nutrients() {
        Ok(nutrient_vec) => nutrient_vec,
        Err(err) => {
            eprintln!("Application error: {}. Is the food.db present?", err);
            process::exit(1)
        }
    };

    println!("{:?}", match_bounds(nutrient_vec, arguments));
}
