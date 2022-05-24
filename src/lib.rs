#![deny(unsafe_code)]

use itertools::Itertools;
use rusqlite::{Connection, Result};
use std::error::Error;

// struct to model the food dataset
#[derive(Debug, Clone)]
pub struct FoodStruct {
    pub name: String,
    pub description: String,
    pub author: String,
    pub ingredients: String,
    pub method: String,
    pub difficulty: String,
    pub img_url: String,
    pub servings: u16,
    pub kcal: u16,
    pub fat: u16,
    pub saturates: u16,
    pub carbs: u16,
    pub sugars: u16,
    pub fibre: u16,
    pub protein: u16,
    pub salt: u16,
}

// Thank you to LegionMammal978#6323 on the Rust Discord server for this function
// this function takes in a dataset, the user's calorie bounds and meal plans and returns a meal
// plan fitted to their needs
pub fn match_bounds(
    nutrient_vec: Vec<FoodStruct>,
    lower_bound: u16,
    upper_bound: u16,
    meal_amnt: usize,
    total_days: usize,
) -> Vec<Vec<(String, u16)>> {
    nutrient_vec
        .iter()
        // get combinations of foods that match the amount of days
        .combinations(meal_amnt)
        // filter out so the foods add up to the user's bounds
        .filter(|combo_arr| {
            let sum = combo_arr.iter().map(|i| i.kcal).sum();
            (lower_bound..upper_bound).contains(&sum)
        })
        .map(|combo_arr| {
            combo_arr
                .into_iter()
                .map(|food| (food.name.clone(), food.kcal))
                .collect()
        })
        // restrict it to the total amount of days the user has requested
        .take(total_days)
        .collect()
}

// this functin uses rusqlite to get the food data from the db
// TODO: implement error handling for db stuff
pub fn def_nutrients() -> Result<Vec<FoodStruct>, Box<dyn Error>> {
    let conn = Connection::open("food.db")?;

    let mut stmt = conn.prepare("SELECT * FROM foodList")?;
    let food_iter = stmt.query_map([], |row| {
        Ok(FoodStruct {
            name: row.get(0)?,
            author: row.get(1)?,
            description: row.get(2)?,
            ingredients: row.get(11)?,
            method: row.get(12)?,
            difficulty: row.get(13)?,
            img_url: row.get(15)?,
            servings: row.get::<_, f64>(14)? as u16,
            kcal: row.get::<_, f64>(3)? as u16,
            fat: row.get::<_, f64>(4)? as u16,
            saturates: row.get::<_, f64>(5)? as u16,
            carbs: row.get::<_, f64>(6)? as u16,
            sugars: row.get::<_, f64>(7)? as u16,
            fibre: row.get::<_, f64>(8)? as u16,
            protein: row.get::<_, f64>(9)? as u16,
            salt: row.get::<_, f64>(10)? as u16,
        })
    })?;

    // get nutrient vector
    let nutrient_vec = food_iter.collect::<Vec<_>>();

    let mut tuple_vec: Vec<FoodStruct> = vec![];

    tuple_vec.extend(nutrient_vec.iter().map(|item| {
        let item = item.as_ref().unwrap();
        FoodStruct {
            name: item.name.clone(),
            author: item.author.clone(),
            description: item.description.clone(),
            ingredients: item.ingredients.clone(),
            method: item.method.clone(),
            difficulty: item.difficulty.clone(),
            img_url: item.img_url.clone(),
            servings: item.servings,
            kcal: item.kcal,
            fat: item.fat,
            saturates: item.saturates,
            carbs: item.carbs,
            sugars: item.sugars,
            fibre: item.fibre,
            protein: item.protein,
            salt: item.salt,
        }
    }));

    Ok(tuple_vec)
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn first_test () {
//        assert_eq!(
//    }
//}