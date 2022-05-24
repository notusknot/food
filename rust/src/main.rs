mod lib;

use lib::*;
use std::env;
use std::process;

fn main() {
    // collect arguments into vec
    let args: Vec<String> = env::args().collect();
    // parse them into Arguments struct or exit if there is an error
    let arguments: Arguments = Arguments::get(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}\n\nUSAGE:\n    food [lower_bound] [upper_bound] [daily_meals] [total_days]", err);
        process::exit(1);
    });

    // get nutrient_vec from db
    let nutrient_vec = def_nutrients().unwrap();

    println!(
        "{:?}",
        // TODO: Find a way to make passing these in cleaner?
        match_bounds(
            nutrient_vec,
            arguments.lower_bound,
            arguments.upper_bound,
            arguments.daily_meals,
            arguments.total_days
        )
    );

    if let Err(e) = def_nutrients() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Arguments {
    lower_bound: u16,
    upper_bound: u16,
    daily_meals: usize,
    total_days: usize,
}

impl Arguments {
    fn get(args: &[String]) -> Result<Arguments, &str> {
        if args.len() - 1 < 4 {
            return Err("not enough arguments");
        }

        if args.len() - 1 > 4 {
            return Err("too many arguments");
        }

        let lower_bound = args[1].clone().parse::<u16>().unwrap();
        let upper_bound = args[2].clone().parse::<u16>().unwrap();
        let daily_meals = args[3].clone().parse::<usize>().unwrap();
        let total_days = args[4].clone().parse::<usize>().unwrap();

        Ok(Arguments {
            lower_bound,
            upper_bound,
            daily_meals,
            total_days,
        })
    }
}
