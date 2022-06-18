#![deny(unsafe_code)]
use std::collections::HashSet;

use fastrand::Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// struct to model the food dataset
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Serialize)]
pub struct DayOfMeals<'a> {
    pub day: Vec<FoodItem<'a>>,
}
#[derive(Debug, Serialize)]
pub struct FinalMealPlan<'a> {
    pub plan: Vec<DayOfMeals<'a>>,
}
pub struct Arguments {
    pub kcal_lower_bound: u16,
    pub kcal_upper_bound: u16,
    pub daily_meals: usize,
    pub total_days: usize,
}

pub fn generate_random_combination<'a, 'b: 'a>(
    rng: &'a Rng,
    nutrients: &'a [FoodItem<'b>],
    amount: usize,
) -> Vec<FoodItem<'b>> {
    let mut combination: Vec<FoodItem<'_>> = vec![];

    // create combination
    for _ in 0..amount {
        let i: usize = rng.usize(..nutrients.len());
        combination.push(nutrients[i]);
    }

    combination
}

// TODO: possibly rewrite this function recursively?
pub fn match_bounds<'a, 'b: 'a>(
    nutrients: &'a [FoodItem<'b>],
    arguments: Arguments,
) -> Result<Vec<Vec<FoodItem<'b>>>, &'static str> {
    let mut matched_list: Vec<Vec<FoodItem<'_>>> = vec![];
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

#[wasm_bindgen]
pub fn match_bounds_json(
    json_data_set: String,
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
    let data_set: Vec<FoodItem<'_>> = serde_json::from_str(&json_data_set).unwrap();
    serde_json::to_string(&match_bounds(&data_set, arguments).unwrap()).unwrap()
}
