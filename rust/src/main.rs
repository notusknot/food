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

//fn match_bounds(
//    nutrient_vec: Vec<i32>,
//    lower_bound: i32,
//    upper_bound: i32,
//    meal_amnt: usize,
//    total_days: usize,
//) -> Vec<Vec<i32>> {
//    nutrient_vec.iter().combinations(meal_amnt).filter(|combo_arr| {
//      let sum: i32 = combo_arr.iter().map(|x| **x).sum();
//      return (lower_bound..upper_bound).contains(&sum);
//    }).take(meal_amnt).collect()
//}

fn get_args() -> (i32, i32, usize, usize) {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let low = &args[1].parse::<i32>().unwrap();
    let up = &args[2].parse::<i32>().unwrap();
    let meals = &args[3].parse::<usize>().unwrap();
    let days = &args[4].parse::<usize>().unwrap();

    return (*low, *up, *meals, *days);
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

fn main() -> Result<()> {
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
    //let nutrient_vec = food_iter.map(|x| x.unwrap()).collect::<Vec<_>>();
    let nutrient_vec = food_iter.collect::<Vec<_>>();
    
    let mut tuple_vec: Vec<(String, i32)> = vec![];

    let length = nutrient_vec.len();
    for nutrient_iterator in 0..length {
        //println!("{:?}", (nutrient_vec[nutrient_iterator].as_ref(), nutrient_vec[nutrient_iterator]));
        tuple_vec.push((
            nutrient_vec[nutrient_iterator].clone().unwrap().name,
            nutrient_vec[nutrient_iterator].clone().unwrap().kcal
        ));
        //tuple_vec.push((nutrient_vec[nutrient_iterator].name.clone(), nutrient_vec[nutrient_iterator].kcal));
    }

    //println!("{:?}", tuple_vec);
    //println!("{:?}", nutrient_vec);

    //let (lower_bound, upper_bound, meal_amnt, total_days) = get_args();

    //println!(
    //    "{:?}",
    //    match_bounds(
    //        get_nutrient_from_struct(&nutrient_vec),
    //        lower_bound,
    //        upper_bound,
    //        meal_amnt,
    //        total_days
    //    )
    //);

    Ok(())
}


