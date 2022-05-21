mod lib;

use lib::*;
use rusqlite::{Connection, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct FoodStruct {
    name: String,
    description: String,
    author: String,
    kcal: u16,
    fat: u16,
    saturates: u16,
    carbs: u16,
    sugars: u16,
    fibre: u16,
    protein: u16,
    salt: u16,
    ingredients: String,
    method: String,
    difficulty: String,
    servings: u16,
    img_url: String,
}

pub fn def_nutrients() -> Result<Vec<(String, u16)>> {
    let conn = Connection::open("food.db")?;

    let mut stmt = conn.prepare("SELECT * FROM foodList")?;
    let food_iter = stmt.query_map([], |row| {
        Ok(FoodStruct {
            name: row.get(0)?,
            author: row.get(1)?,
            description: row.get(2)?,
            kcal: row.get::<_, f64>(3)? as u16,
            fat: row.get::<_, f64>(4)? as u16,
            saturates: row.get::<_, f64>(5)? as u16,
            carbs: row.get::<_, f64>(6)? as u16,
            sugars: row.get::<_, f64>(7)? as u16,
            fibre: row.get::<_, f64>(8)? as u16,
            protein: row.get::<_, f64>(9)? as u16,
            salt: row.get::<_, f64>(10)? as u16,
            ingredients: row.get(11)?,
            method: row.get(12)?,
            difficulty: row.get(13)?,
            servings: row.get::<_, f64>(14)? as u16,
            img_url: row.get(15)?,
        })
    })?;

    // Get nutrient vector
    let nutrient_vec = food_iter.collect::<Vec<_>>();

    let mut tuple_vec: Vec<(String, u16)> = vec![];

    let length = nutrient_vec.len();
    for nutrient_iterator in 0..length {
        tuple_vec.push((
            nutrient_vec[nutrient_iterator]
                .as_ref()
                .unwrap()
                .name
                .clone(),
            nutrient_vec[nutrient_iterator]
                .as_ref()
                .unwrap()
                .kcal
                .clone(),
        ));
    }
    Ok(tuple_vec)
}

fn get_args() -> (u16, u16, usize, usize) {
    let help_msg = "USAGE: food [lower_bound] [upper_bound] [daily_meals] [total_days]";

    let error_msg = "Arguments must be positive integers and within range";

    let args: Vec<String> = env::args().collect();

    let low = args
        .get(1)
        .expect(help_msg)
        .parse::<u16>()
        .expect(error_msg);
    let up = args
        .get(2)
        .expect(help_msg)
        .parse::<u16>()
        .expect(error_msg);
    let meals = args
        .get(3)
        .expect(help_msg)
        .parse::<usize>()
        .expect(error_msg);
    let days = args
        .get(4)
        .expect(help_msg)
        .parse::<usize>()
        .expect(error_msg);

    return (low, up, meals, days);
}

fn main() {
    let tuple_vec = def_nutrients().unwrap();
    let (lower_bound, upper_bound, meal_amnt, total_days) = get_args();

    println!(
        "{:?}",
        match_bounds(
            tuple_vec,
            lower_bound.try_into().unwrap(),
            upper_bound.try_into().unwrap(),
            meal_amnt,
            total_days
        )
    );
}
