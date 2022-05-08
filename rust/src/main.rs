use itertools::Itertools;
use rusqlite::{Connection, Result};
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

fn get_args() -> (i32, i32, usize, usize) {
    let args: Vec<String> = env::args().collect();

    let low = &args[1].parse::<i32>().unwrap();
    let up = &args[2].parse::<i32>().unwrap();
    let meals = &args[3].parse::<usize>().unwrap();
    let days = &args[4].parse::<usize>().unwrap();

    return (*low, *up, *meals, *days);
}

//fn fake_def_nutrients() -> Vec<i32> {
//    let mut nutrient_vec = vec![];
//    for req in 550..600 {
//        nutrient_vec.push(req);
//    }
//
//    // Iterate from middle out
//    fn cycle<T>(slice: &[T], start_pos: usize) -> impl Iterator<Item = &T> {
//        slice.iter().cycle().skip(start_pos).take(slice.len())
//    }
//
//    let nutrient_vec = cycle(&nutrient_vec, nutrient_vec.len() / 3)
//        .copied()
//        .collect::<Vec<_>>();
//
//    nutrient_vec
//}

#[derive(Debug)]
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

fn nutrient_names_owned(kcal: &[FoodStruct]) -> Vec<i32> {
    kcal.iter().map(|p| p.kcal.clone()).collect()
}

fn main() -> Result<()> {
    let conn = Connection::open("food.db")?;
    let mut nutrient_vec: Vec<FoodStruct> = vec![];

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

    for item in food_iter {
        nutrient_vec.push(item.unwrap());
    }

    let lower_bound = get_args().0;
    let upper_bound = get_args().1;
    let meal_amnt = get_args().2;
    let total_days = get_args().3;
    println!("{:?}", match_bounds(nutrient_names_owned(&nutrient_vec), lower_bound, upper_bound, meal_amnt, total_days));

    Ok(())
}
