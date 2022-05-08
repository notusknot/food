use itertools::Itertools;
use rusqlite::{params, Connection, Result};
use std::env;

// thank you to Candy Corvid#0137 for helping me with this!
fn match_bounds(
    nutrient_vec: Vec<i32>,
    lower_bound: i32,
    upper_bound: i32,
    meal_amnt: usize,
    total_days: usize,
) -> Vec<Vec<i32>> {
    let mut matched_list: Vec<Vec<i32>> = vec![];

    for combo_arr in nutrient_vec.into_iter().combinations(meal_amnt) {
        let sum: i32 = combo_arr.iter().sum();

        if (lower_bound..upper_bound).contains(&sum) {
            matched_list.push(combo_arr.clone());
            if matched_list.len() == total_days {
                return matched_list;
            }
        }
    }

    matched_list
}

fn fake_def_nutrients() -> Vec<i32> {
    let mut nutrient_vec = vec![];
    for req in 550..600 {
        nutrient_vec.push(req);
    }

    // Iterate from middle out
    fn cycle<T>(slice: &[T], start_pos: usize) -> impl Iterator<Item = &T> {
        slice.iter().cycle().skip(start_pos).take(slice.len())
    }

    let nutrient_vec = cycle(&nutrient_vec, nutrient_vec.len() / 3)
        .copied()
        .collect::<Vec<_>>();

    nutrient_vec
}

fn get_args() -> (i32, i32, usize, usize) {
    let args: Vec<String> = env::args().collect();

    let low = &args[1].parse::<i32>().unwrap();
    let up = &args[2].parse::<i32>().unwrap();
    let meals = &args[3].parse::<usize>().unwrap();
    let days = &args[4].parse::<usize>().unwrap();

    return (*low, *up, *meals, *days);
}

#[derive(Debug)]
struct FoodStruct {
    name: String,
    description: String,
    author: String,
    kcal: f64,
    fat: f64,
    saturates: f64,
    carbs: f64,
    sugars: f64,
    fibre: f64,
    protein: f64,
    salt: f64,
    ingredients: String,
    method: String,
    difficulty: String,
    servings: i32,
    img_url: String,
}

fn def_nutrients() -> Result<()> {
    let conn = Connection::open("food.db").unwrap();
    let mut nutrient_vec: Vec<FoodStruct> = vec![];

    let mut stmt = conn.prepare("SELECT * FROM foodList")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(FoodStruct {
            name: row.get(0)?,
            description: row.get(1)?,
            author: row.get(2)?,
            kcal: row.get(3)?,
            fat: row.get(4)?,
            saturates: row.get(5)?,
            carbs: row.get(6)?,
            sugars: row.get(7)?,
            fibre: row.get(8)?,
            protein: row.get(9)?,
            salt: row.get(10)?,
            ingredients: row.get(11)?,
            method: row.get(12)?,
            difficulty: row.get(13)?,
            servings: row.get(14)?,
            img_url: row.get(15)?,
        })
    })?;

    for person in person_iter {
        //nutrient_vec.push(person.as_ref().unwrap());
        //println!("{:?}", person.unwrap());
    }
    //println!("{:?}", nutrient_vec);
    Ok(())
}

fn main() {
    //let lower_bound = get_args().0;
    //let upper_bound = get_args().1;
    //let meal_amnt = get_args().2;
    //let total_days = get_args().3;
    //let nutrient_vec = fake_def_nutrients();
    //println!("{:?}", match_bounds(nutrient_vec, lower_bound, upper_bound, meal_amnt, total_days))
    println!("{:?}", def_nutrients());
}
