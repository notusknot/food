pub mod data;
pub mod lib;

use data::*;
use lib::*;
use std::{env, process};

impl Arguments {
    pub fn get(args: &[String]) -> Result<Arguments, &str> {
        if args.len() - 1 < 4 {
            return Err("not enough arguments");
        }

        if args.len() - 1 > 4 {
            return Err("too many arguments");
        }

        let kcal_lower_bound = args[1].parse::<u16>().map_err(|_| "invalid number")?;
        let kcal_upper_bound = args[2].parse::<u16>().map_err(|_| "invalid number")?;
        let daily_meals = args[3].parse::<usize>().map_err(|_| "invalid number")?;
        let total_days = args[4].parse::<usize>().map_err(|_| "invalid number")?;

        Ok(Arguments {
            kcal_lower_bound,
            kcal_upper_bound,
            daily_meals,
            total_days,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // parse args into Arguments struct or exit if there is an error
    let arguments: Arguments = Arguments::get(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}\n\nUSAGE:\n    food [kcal_lower_bound] [kcal_upper_bound] [daily_meals] [total_days]", err);
        process::exit(1);
    });

    // get meal plan from match_bounds
    let meal_plan: FinalMealPlan = match match_bounds(NUTRIENTS, arguments) {
        Ok(meal_plan) => meal_plan,
        Err(error) => {
            eprintln!("Application error: {}", error);
            process::exit(1)
        }
    };

    println!("{}", meal_plan);
}
