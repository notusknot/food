#![deny(unsafe_code)]

use itertools::Itertools;
use std::collections::HashSet;
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

pub struct Arguments {
    pub lower_bound: u16,
    pub upper_bound: u16,
    pub daily_meals: usize,
    pub total_days: usize,
}

impl Arguments {
    pub fn get(args: &[String]) -> Result<Arguments, &str> {
        if args.len() - 1 < 4 {
            return Err("not enough arguments");
        }

        if args.len() - 1 > 4 {
            return Err("too many arguments");
        }

        let lower_bound = args[1].parse::<u16>().map_err(|_| "invalid number")?;
        let upper_bound = args[2].parse::<u16>().map_err(|_| "invalid number")?;
        let daily_meals = args[3].parse::<usize>().map_err(|_| "invalid number")?;
        let total_days = args[4].parse::<usize>().map_err(|_| "invalid number")?;

        Ok(Arguments {
            lower_bound,
            upper_bound,
            daily_meals,
            total_days,
        })
    }
}

// Thank you to LegionMammal978#6323 on the Rust Discord server for this function
// this function takes in a dataset, the user's calorie bounds and meal plans and returns a meal
// plan fitted to their needs
pub fn match_bounds(
    nutrient_vec: Vec<FoodStruct>,
    arguments: Arguments,
    // super messy but it works
    // TODO: figure out how to do this with a struct, not a long tuple!!
) -> Vec<Vec<(String, String, String, String, String, u16, u16, u16, u16)>> {
    let mut seen = HashSet::new();

    nutrient_vec
        .iter()
        // get combinations of foods that match the amount of days
        .combinations(arguments.daily_meals)
        // filter out so the foods add up to the user's bounds
        .filter(|combo_arr| {
            let sum = combo_arr.iter().map(|i| i.kcal).sum();
            (arguments.lower_bound..arguments.upper_bound).contains(&sum)
        })
        .map(|combo_arr| {
            combo_arr
                .iter()
                .map(|food| {
                    (
                        food.name.clone(),
                        food.method.clone(),
                        food.ingredients.clone(),
                        food.difficulty.clone(),
                        food.img_url.clone(),
                        food.kcal,
                        food.fat,
                        food.protein,
                        food.carbs,
                    )
                })
                .collect()
        })
        // filter duplicates
        .filter(|v: &Vec<_>| {
            v.iter()
                .all(|(name, _, _, _, _, _, _, _, _)| seen.insert(name.clone()))
        })
        // restrict it to the total amount of days the user has requested
        .take(arguments.total_days)
        .collect()
}

// this functin uses rusqlite to get the food data from the db
// TODO: implement error handling for db stuff
pub fn def_nutrients() -> Result<Vec<FoodStruct>, Box<dyn Error>> {
    let connection = sqlite::open("food.db").unwrap();

    let mut cursor = connection
        .prepare("SELECT * FROM foodList ORDER BY RANDOM() limit 100")
        .unwrap()
        .into_cursor();

    let mut food_struct_vec: Vec<FoodStruct> = vec![];

    while let Some(row) = cursor.next().unwrap() {
        let temporary_food_struct = FoodStruct {
            name: row[0].as_string().unwrap().to_string(),
            author: row[1].as_string().unwrap().to_string(),
            description: row[2].as_string().unwrap().to_string(),
            ingredients: row[11].as_string().unwrap().to_string(),
            method: row[12].as_string().unwrap().to_string(),
            difficulty: row[13].as_string().unwrap().to_string(),
            img_url: row[15].as_string().unwrap().to_string(),
            servings: row[14].as_integer().unwrap() as u16,
            kcal: row[3].as_float().unwrap() as u16,
            fat: row[4].as_float().unwrap() as u16,
            saturates: row[5].as_float().unwrap() as u16,
            carbs: row[6].as_float().unwrap() as u16,
            sugars: row[7].as_float().unwrap() as u16,
            fibre: row[8].as_float().unwrap() as u16,
            protein: row[9].as_float().unwrap() as u16,
            salt: row[10].as_float().unwrap() as u16,
        };

        food_struct_vec.push(temporary_food_struct);
    }

    Ok(food_struct_vec)
}
