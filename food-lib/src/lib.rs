use itertools::Itertools;
use rusqlite::{Connection, Result};
use std::env;

// Thank you to LegionMammal978#6323 on the Rust Discord server for this function
pub fn match_bounds(
    nutrient_vec: Vec<(String, i32)>,
    lower_bound: i32,
    upper_bound: i32,
    meal_amnt: usize,
    total_days: usize,
) -> Vec<Vec<(String, i32)>> {
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

#[derive(Clone, Debug)]
struct FoodStruct {
    name: String,
    description: String,
    author: String,
    kcal: i32,
    fat: i32,
    saturates: i32,
    carbs: i32,
    sugars: i32,
    fibre: i32,
    protein: i32,
    salt: i32,
    ingredients: String,
    method: String,
    difficulty: String,
    servings: i32,
    img_url: String,
}

pub fn fetch_data() -> Result<()> {
    let conn = Connection::open("food.db")?;

    let mut stmt = conn.prepare("SELECT * FROM foodList")?;
    let food_iter = stmt.query_map([], |row| {
        Ok(FoodStruct {
            name: row.get(0)?,
            author: row.get(1)?,
            description: row.get(2)?,
            kcal: row.get::<_, f64>(3)? as i32,
            fat: row.get::<_, f64>(4)? as i32,
            saturates: row.get::<_, f64>(5)? as i32,
            carbs: row.get::<_, f64>(6)? as i32,
            sugars: row.get::<_, f64>(7)? as i32,
            fibre: row.get::<_, f64>(8)? as i32,
            protein: row.get::<_, f64>(9)? as i32,
            salt: row.get::<_, f64>(10)? as i32,
            ingredients: row.get(11)?,
            method: row.get(12)?,
            difficulty: row.get(13)?,
            servings: row.get::<_, f64>(14)? as i32,
            img_url: row.get(15)?,
        })
    })?;

    // Get nutrient vector
    let nutrient_vec = food_iter.collect::<Vec<_>>();

    let mut tuple_vec: Vec<(String, i32)> = vec![];

    let length = nutrient_vec.len();
    for nutrient_iterator in 0..length {
        tuple_vec.push((
            nutrient_vec[nutrient_iterator].as_ref().unwrap().name.clone(),
            nutrient_vec[nutrient_iterator].as_ref().unwrap().kcal.clone()
        ));
    }

    let (lower_bound, upper_bound, meal_amnt, total_days) = get_args();

    println!(
        "{:?}",
        match_bounds(
            tuple_vec,
            lower_bound,
            upper_bound,
            meal_amnt,
            total_days
        )
    );

    Ok(())
}
