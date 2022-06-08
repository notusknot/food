#![deny(unsafe_code)]

use itertools::Itertools;
//use miniserde::{json, Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

// struct to model the food dataset
#[derive(Debug, Clone)]
pub struct FoodStruct {
    pub name: String,
    pub description: String,
    pub author: String,
    pub ingredients: String,
    pub instructions: String,
    pub difficulty: String,
    pub img_url: String,
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

impl fmt::Display for FoodStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so what's needed is printed
        write!(f, "name: {}, \ningredients: {}, \ninstructions: {}, \nservings: {}, \nkcal: {}, \nfat: {}, \ncarbs: {}, \nprotein: {}\n", 
        self.name, self.ingredients, self.instructions, self.servings, self.kcal, self.fat, self.carbs, self.protein)
    }
}

#[derive(Debug)]
pub struct MealsForDay( pub Vec<FoodStruct> );

impl fmt::Display for MealsForDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| writeln!(f, "{}", food))
        })
    }
}

#[derive(Debug)]
pub struct FinalMealPlan( pub Vec<MealsForDay> );

impl fmt::Display for FinalMealPlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| writeln!(f, "New day\n-------------------\n{}", food))
        })
    }
}

pub struct Arguments {
    pub kcal_lower_bound: u16,
    pub kcal_upper_bound: u16,
    pub protein_lower_bound: u16,
    pub protein_upper_bound: u16,
    pub daily_meals: usize,
    pub total_days: usize,
}

impl Arguments {
    pub fn get(args: &[String]) -> Result<Arguments, &str> {
        if args.len() - 1 < 6 {
            return Err("not enough arguments");
        }

        if args.len() - 1 > 6 {
            return Err("too many arguments");
        }

        let kcal_lower_bound = args[1].parse::<u16>().map_err(|_| "invalid number")?;
        let kcal_upper_bound = args[2].parse::<u16>().map_err(|_| "invalid number")?;
        let protein_lower_bound = args[3].parse::<u16>().map_err(|_| "invalid number")?;
        let protein_upper_bound = args[4].parse::<u16>().map_err(|_| "invalid number")?;
        let daily_meals = args[5].parse::<usize>().map_err(|_| "invalid number")?;
        let total_days = args[6].parse::<usize>().map_err(|_| "invalid number")?;

        Ok(Arguments {
            kcal_lower_bound,
            kcal_upper_bound,
            protein_lower_bound,
            protein_upper_bound,
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
) -> FinalMealPlan {
    let mut seen = HashSet::new();

    FinalMealPlan(nutrient_vec
        .into_iter()
        // get combinations of foods that match the amount of days
        .combinations(arguments.daily_meals)
        // filter out so the foods add up to the user's bounds
        .filter(|combo_arr| {
            let sum = combo_arr.iter().map(|i| i.kcal).sum();
            //let protein_sum = combo_arr.iter().map(|i| i.protein).sum();
            (arguments.kcal_lower_bound..arguments.kcal_upper_bound).contains(&sum)
            //.contains((arguments.protein_lower_bound..arguments.protein_upper_bound).contains(&protein_sum))
        })
        // filter duplicates
        .filter(|v: &Vec<_>| {
            v.iter().all(|food| {
                //if !seen.contains(&food.name) {
                seen.insert(food.name.clone())
                //}
            })
        })
        // restrict it to the total amount of days the user has requested
        .take(arguments.total_days)
        .map(MealsForDay)
        .collect::<Vec<MealsForDay>>())
}
/*
pub fn match_bounds_json(
    json_data_set: &str,
    kcal_lower_bound: u16,
    kcal_upper_bound: u16,
    protein_lower_bound: u16,
    protein_upper_bound: u16,
    daily_meals: usize,
    total_days: usize,
) -> String {
    let data_set: Vec<FoodStruct> = json::from_str(json_data_set).unwrap();
    let arguments = Arguments {
        kcal_lower_bound,
        kcal_upper_bound,
        protein_lower_bound,
        protein_upper_bound,
        daily_meals,
        total_days,
    };
    let nutrient_vec = match_bounds(data_set, arguments);
    json::to_string(&nutrient_vec)
}
*/