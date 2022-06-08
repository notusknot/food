mod lib;

use lib::*;
use std::error::Error;
use std::{env, process};

fn main() {
    // collect arguments into vec
    let args: Vec<String> = env::args().collect();
    // parse them into Arguments struct or exit if there is an error
    let arguments: Arguments = Arguments::get(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}\n\nUSAGE:\n    food [kcal_lower_bound] [kcal_upper_bound] [protein_lower_bound] [protein_upper_bound] [daily_meals] [total_days]", err);
        process::exit(1);
    });

    // get nutrient_vec from db
    let nutrient_vec = match def_nutrients() {
        Ok(nutrient_vec) => nutrient_vec,
        Err(err) => {
            eprintln!("Application error: {}. Is the food.db present?", err);
            process::exit(1)
        }
    };

    println!("{}", match_bounds(nutrient_vec, arguments));
}

// this functin uses rusqlite to get the food data from the db
// TODO: implement error handling for db stuff
pub fn def_nutrients() -> Result<Vec<FoodStruct>, Box<dyn Error>> {
    let connection = sqlite::open("food.db").unwrap();

    let mut cursor = connection
        .prepare("SELECT * FROM foodList ORDER BY RANDOM() limit 100")
        .unwrap()
        .into_cursor();

    let mut food_struct_list: Vec<FoodStruct> = vec![];

    while let Some(row) = cursor.next().unwrap() {
        let temporary_food_struct = FoodStruct {
            name: row[0].as_string().unwrap().to_string(),
            author: row[1].as_string().unwrap().to_string(),
            description: row[2].as_string().unwrap().to_string(),
            ingredients: row[11].as_string().unwrap().to_string(),
            instructions: row[12].as_string().unwrap().to_string(),
            difficulty: row[13].as_string().unwrap().to_string(),
            img_url: row[15].as_string().unwrap().to_string(),
            servings: row[14].as_integer().unwrap() as u16,
            kcal: row[3].as_float().unwrap() as u16,
            fat: row[4].as_float().unwrap() as u16,
            saturates: row[5].as_float().unwrap() as u16,
            carbs: row[6].as_float().unwrap() as u16,
            sugars: row[7].as_float().unwrap() as u16,
            fiber: row[8].as_float().unwrap() as u16,
            protein: row[9].as_float().unwrap() as u16,
            salt: row[10].as_float().unwrap() as u16,
        };

        food_struct_list.push(temporary_food_struct);
    }

    Ok(food_struct_list)
}