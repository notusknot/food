pub mod data;

use data::*;
use food_lib::*;
use std::process;

const HELP: &str = "\
food: generate unique meal plans to fit your needs

USAGE:
    food [kcal_lower_bound] [kcal_upper_bound] [daily_meals] [total_days] [OPTIONS]

ARGS:
    <kcal_lower_bound>    The minimum calories per day
    <kcal_upper_bound>    The maximum calories per day
    <daily_meals>         The amount of meals per day
    <total_days>          The total amount of days to generate

OPTIONS:
    -j --json        Output JSON (not implemented yet)

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints current version
";

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // parse args into CliArgs struct or exit if there is an error
    let arguments: CliArgs = match CliArgs::parse_args(HELP, VERSION) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", HELP);
            eprintln!("\x1b[1;31mProblem parsing arguments: {}", e);
            std::process::exit(1);
        }
    };

    let meal_plan = match match_bounds(NUTRIENTS, arguments) {
        Ok(meal_plan) => meal_plan,
        Err(error) => {
            eprintln!("Application error: {}", error);
            process::exit(1)
        }
    };

    let final_meal_plan = FinalMealPlan {
        plan: meal_plan
            .into_iter()
            .map(|day| DayOfMeals { day })
            .collect(),
    };

    println!("{}", final_meal_plan);
}
