#![deny(unsafe_code)]

use std::collections::HashSet;
use std::fmt;

use fastrand::Rng;
use miniserde::{json::from_str, Serialize, Deserialize};
use wasm_bindgen::prelude::*;

// struct to model the food dataset
#[derive(Debug, Clone, Serialize, )]
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

#[derive(Debug, Serialize)]
pub struct DayOfMeals{
    pub day: Vec<&'static FoodItem>
}
#[derive(Debug, Serialize)]
pub struct FinalMealPlan{
    pub plan: Vec<DayOfMeals>,
}
pub struct Arguments {
    pub kcal_lower_bound: u16,
    pub kcal_upper_bound: u16,
    pub daily_meals: usize,
    pub total_days: usize,
}

pub fn generate_random_combination(
    rng: &Rng,
    nutrients: &'static [FoodItem],
    amount: usize,
) -> Vec<&'static FoodItem> {
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
) -> Result<Vec<Vec<&'static FoodItem>>, &str> {
    let mut matched_list: Vec<Vec<&'static FoodItem>> = vec![];
    let mut seen: HashSet<&str> = HashSet::new();

    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // max amount of tries to prevent the program from running endlessly
    const TRIES: usize = 100_000;
    let rng = fastrand::Rng::new();

    // iterates through each combination of meals
    for _ in 0..TRIES {
        let combination = generate_random_combination(&rng, nutrients, arguments.daily_meals);

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

/*
pub fn to_json(final_meal_plan: FinalMealPlan) -> String {
    use miniserde::json::*;
    to_string(&final_meal_plan)
}

pub fn match_bounds_json(
    json_data_set: &str,
    kcal_lower_bound: u16,
    kcal_upper_bound: u16,
    daily_meals: usize,
    total_days: usize,
) -> String {
    let arguments = Arguments {
        kcal_lower_bound,
        kcal_upper_bound,
        daily_meals,
        total_days,
    };
    let data_set: Vec<FoodItem> = from_str(json_data_set).unwrap();
    let nutrient_vec = match_bounds(&data_set, arguments).unwrap();
    miniserde::json::to_string(&nutrient_vec)
}

*/