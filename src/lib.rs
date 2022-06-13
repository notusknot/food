#![deny(unsafe_code)]

use std::collections::HashSet;
use std::fmt;

use fastrand::Rng;

// struct to model the food dataset
#[derive(Debug, Clone)]
pub struct FoodItem {
    pub name: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub ingredients: &'static str,
    pub instructions: &'static str,
    pub difficulty: &'static str,
    pub img_url: &'static str,
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
impl fmt::Display for FoodItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so what's needed is printed
        write!(f, "name: {}, \ningredients: {}, \ninstructions: {}, \nservings: {}, \nkcal: {}, \nfat: {}, \ncarbs: {}, \nprotein: {}\n", 
        self.name, self.ingredients, self.instructions, self.servings, self.kcal, self.fat, self.carbs, self.protein)
    }
}

#[derive(Debug)]
pub struct DayOfMeals(pub Vec<&'static FoodItem>);

impl fmt::Display for DayOfMeals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| writeln!(f, "{}", food))
        })
    }
}

#[derive(Debug)]
pub struct FinalMealPlan(pub Vec<DayOfMeals>);

impl fmt::Display for FinalMealPlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, food| {
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

pub struct Arguments {
    pub kcal_lower_bound: u16,
    pub kcal_upper_bound: u16,
    pub daily_meals: usize,
    pub total_days: usize,
}

pub fn random_combinations(rng: &Rng, nutrients: &'static [FoodItem], amount: usize) -> Vec<&'static FoodItem> {
    let mut combination: Vec<&'static FoodItem> = vec![];

    // create combination
    for _ in 0..amount {
        let i: usize = rng.usize(..nutrients.len());
        combination.push(&nutrients[i]);
    }

    combination
}

// TODO: possibly rewrite this function recursively?
pub fn match_bounds(
    nutrients: &'static [FoodItem],
    arguments: Arguments,
) -> Result<FinalMealPlan, &str> {
    let mut matched_list: Vec<DayOfMeals> = vec![];
    let mut seen: HashSet<&str> = HashSet::new();

    // max amount of tries to prevent the program from running endlessly
    const TRIES: usize = 100_000;
    let rng = fastrand::Rng::new();

    // iterates through each combination of meals
    for _ in 0..TRIES {
        let combination = random_combinations(&rng, nutrients, arguments.daily_meals);

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
        matched_list.push(DayOfMeals(combination));

        // return when we reach the amount of days the user requested
        if matched_list.len() == arguments.total_days {
            return Ok(FinalMealPlan(matched_list));
        }
    }

    // if the loop can't find a combination within the amount of tries, fail
    Err("Could not create satisfactory meal plan. Maybe tweak the parameters?")
}
