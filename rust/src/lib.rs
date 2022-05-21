use itertools::Itertools;
use miniserde::json::from_str;
use miniserde::json::to_string;
use wasm_bindgen::prelude::*;

// Thank you to LegionMammal978#6323 on the Rust Discord server for this function
pub fn match_bounds(
    nutrient_vec: Vec<(String, u16)>,
    lower_bound: u16,
    upper_bound: u16,
    meal_amnt: usize,
    total_days: usize,
) -> Vec<Vec<(String, u16)>> {
    nutrient_vec
        .iter()
        .combinations(meal_amnt)
        .filter(|combo_arr| {
            let sum = combo_arr.iter().map(|i| i.1).sum();
            (lower_bound..upper_bound).contains(&sum)
        })
        .map(|combo_arr| combo_arr.into_iter().cloned().collect())
        .take(total_days)
        .collect()
}

#[wasm_bindgen]
pub fn match_bounds_json(
    data_set: String,
    lower_bound: u16,
    upper_bound: u16,
    meal_amnt: usize,
    total_days: usize,
) -> String {
    let tuple_vec: Vec<(String, u16)> = from_str(&data_set.as_str()).unwrap();
    let nutrient_vec = match_bounds(tuple_vec, lower_bound, upper_bound, meal_amnt, total_days);
    let json = to_string(&nutrient_vec);

    json
}
