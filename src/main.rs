pub mod data;
pub mod lib;

use data::*;
use lib::*;
use std::fmt;
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
    // parse args into Arguments struct or exit if there is an error
    let arguments: Arguments = match Arguments::parse_args() {
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

impl Arguments {
    fn parse_args() -> Result<Arguments, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        // Help has a higher priority and should be handled separately.
        if pargs.contains(["-h", "--help"]) {
            eprint!("{}", HELP);
            std::process::exit(0);
        }

        if pargs.contains(["-v", "--version"]) {
            eprintln!("{}", VERSION);
            std::process::exit(0);
        }

        if pargs.contains(["-j", "--json"]) {
            eprintln!("\x1b[1;31mJSON output has not been implemented yet");
        }

        let args = Arguments {
            kcal_lower_bound: pargs.free_from_str()?,
            kcal_upper_bound: pargs.free_from_str()?,
            daily_meals: pargs.free_from_str()?,
            total_days: pargs.free_from_str()?,
        };

        // It's up to the caller what to do with the remaining arguments.
        let remaining = pargs.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unused arguments left: {:?}.", remaining);
        }

        Ok(args)
    }
}

impl fmt::Display for FoodItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so what's needed is printed
        write!(f, "name: {}, \ningredients: {}, \ninstructions: {}, \nservings: {}, \nkcal: {}, \nfat: {}, \ncarbs: {}, \nprotein: {}\n", 
        self.name, self.ingredients, self.instructions, self.servings, self.kcal, self.fat, self.carbs, self.protein)
    }
}

impl fmt::Display for DayOfMeals<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.day.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| writeln!(f, "{}", food))
        })
    }
}

impl fmt::Display for FinalMealPlan<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.plan.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| {
                writeln!(
                    f,
                    "New Day:\n────────────────────────────────────────────\n{}",
                    food
                )
            })
        })
    }
}
