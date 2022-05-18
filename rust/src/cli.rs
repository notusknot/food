mod lib;

use std::env;
use lib::*;

fn get_args() -> (i32, i32, usize, usize) {
    let args: Vec<String> = env::args().collect();

    let low = &args[1].parse::<i32>().unwrap();
    let up = &args[2].parse::<i32>().unwrap();
    let meals = &args[3].parse::<usize>().unwrap();
    let days = &args[4].parse::<usize>().unwrap();

    return (*low, *up, *meals, *days);
}

fn main() {
    let tuple_vec = def_nutrients().unwrap();
    let (lower_bound, upper_bound, meal_amnt, total_days) = get_args();

    println!(
        "{:#?}",
        match_bounds(tuple_vec, lower_bound, upper_bound, meal_amnt, total_days)
    );
}
