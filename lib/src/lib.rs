#![deny(unsafe_code)]
use std::collections::HashSet;

use std::fmt;

// struct to model the food dataset
#[derive(Debug, Clone, Copy)]
pub struct FoodItem<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub author: &'a str,
    pub ingredients: &'a str,
    pub instructions: &'a str,
    pub difficulty: &'a str,
    pub img_url: &'a str,
    pub servings: u16,
    pub kcal: u16,
    pub fat: u16,
    pub saturates: u16,
    pub carbs: u16,
    pub sugars: u16,
    pub fiber: u16,
    pub protein: u16,
    pub salt: u16,
}

/*
Unfortunately, since Rust doesn't allow us to impl on a foregin type,
I have to put the Displays here.
*/
impl fmt::Display for FoodItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so what's needed is printed
        write!(f, "name: {}, \ningredients: {}, \ninstructions: {}, \nservings: {}, \nkcal: {}, \nfat: {}, \ncarbs: {}, \nprotein: {}\n", 
        self.name, self.ingredients, self.instructions, self.servings, self.kcal, self.fat, self.carbs, self.protein)
    }
}

#[derive(Debug)]
pub struct DayOfMeals<'a> {
    pub day: Vec<FoodItem<'a>>,
}

impl fmt::Display for DayOfMeals<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.day.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| writeln!(f, "{}", food))
        })
    }
}

#[derive(Debug)]
pub struct FinalMealPlan<'a> {
    pub plan: Vec<DayOfMeals<'a>>,
}

impl fmt::Display for FinalMealPlan<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.plan.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| {
                writeln!(
                    f,
                    "New Day:\n──────────────────────────────────────────\n{}",
                    food
                )
            })
        })
    }
}

pub struct CliArgs {
    pub kcal_lower_bound: u16,
    pub kcal_upper_bound: u16,
    pub daily_meals: usize,
    pub total_days: usize,
}

/*
Once again, even though this will only ever be used in the cli,
it has to go in here because I can't put an impl on a foreign type.
This is why the library depends on pico_args.
*/
impl CliArgs {
    pub fn parse_args(help_message: &str, version: &str) -> Result<CliArgs, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        // Help has a higher priority and should be handled separately.
        if pargs.contains(["-h", "--help"]) {
            eprint!("{}", help_message);
            std::process::exit(0);
        }

        if pargs.contains(["-v", "--version"]) {
            eprintln!("{}", version);
            std::process::exit(0);
        }

        if pargs.contains(["-j", "--json"]) {
            eprintln!("\x1b[1;31mJSON output has not been implemented yet");
        }

        let args = CliArgs {
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

pub fn random_combination<'a, 'b: 'a>(
    nutrients: &'a [FoodItem<'b>],
    amount: usize,
) -> Vec<FoodItem<'b>> {
    let mut combination: Vec<FoodItem<'_>> = vec![];

    use nanorand::{RNG, WyRand};
    let mut rng = WyRand::new();

    // create combination
    for _ in 0..amount {
        let i: usize = rng.generate_range(1_usize, nutrients.len());
        combination.push(nutrients[i]);
    }

    combination
}


/*
TODO: possibly rewrite this function recursively?
This is the main function that actually creates the meal plan.
*/
pub fn match_bounds<'a, 'b: 'a>(
    nutrients: &'a [FoodItem<'b>],
    arguments: CliArgs,
) -> Result<Vec<Vec<FoodItem<'b>>>, &'static str> {
    let mut matched_list: Vec<Vec<FoodItem<'_>>> = vec![];
    let mut seen: HashSet<&str> = HashSet::new();

    // max amount of tries to prevent the program from running endlessly
    const TRIES: usize = 100_000;

    // iterates through each combination of meals
    for _ in 0..TRIES {
        let combination = random_combination(nutrients, arguments.daily_meals);

        // checks if combination fits within calorie range
        let sum: u16 = combination.iter().map(|i| i.kcal).sum();
        if !(arguments.kcal_lower_bound..arguments.kcal_upper_bound).contains(&sum) {
            continue;
        }

        // marks food as used
        for food in &combination {
            seen.insert(food.name);
        }

        // if all of the previous are successful, add this combination to our list
        matched_list.push(combination);

        // return when we reach the amount of days the user requested
        if matched_list.len() == arguments.total_days {
            return Ok(matched_list);
        }
    }

    // if the loop can't find a combination within the amount of tries, fail
    Err("Could not create satisfactory meal plan. Maybe tweak the parameters?")
}
