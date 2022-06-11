#![deny(unsafe_code)]

use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;

// struct to model the food dataset
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FoodItem {
    pub name: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub ingredients: &'static str,
    pub instructions: &'static str,
    pub difficulty: &'static str,
    pub img_url: &'static str,
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
impl fmt::Display for FoodItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so what's needed is printed
        write!(f, "name: {}, \ningredients: {}, \ninstructions: {}, \nservings: {}, \nkcal: {}, \nfat: {}, \ncarbs: {}, \nprotein: {}\n", 
        self.name, self.ingredients, self.instructions, self.servings, self.kcal, self.fat, self.carbs, self.protein)
    }
}

#[derive(Debug)]
pub struct DayOfMeals( pub Vec<FoodItem> );

impl fmt::Display for DayOfMeals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| writeln!(f, "{}", food))
        })
    }
}

#[derive(Debug)]
pub struct FinalMealPlan( pub Vec<DayOfMeals> );

impl fmt::Display for FinalMealPlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, food| {
            result.and_then(|_| writeln!(f, "New Day:\n────────────────────────────────────────────\n{}", food))
        })
    }
}

pub struct Arguments {
    pub kcal_lower_bound: u16,
    pub kcal_upper_bound: u16,
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

        let kcal_lower_bound = args[1].parse::<u16>().map_err(|_| "invalid number")?;
        let kcal_upper_bound = args[2].parse::<u16>().map_err(|_| "invalid number")?;
        let daily_meals = args[3].parse::<usize>().map_err(|_| "invalid number")?;
        let total_days = args[4].parse::<usize>().map_err(|_| "invalid number")?;

        Ok(Arguments {
            kcal_lower_bound,
            kcal_upper_bound,
            daily_meals,
            total_days,
        })
    }
}
/*
pub fn match_bounds(
    nutrient_vec: &[FoodItem],
    arguments: Arguments,
) -> Vec<Vec<&FoodItem>> {
    let mut matched_list: Vec<Vec<&FoodItem>> = vec![];
    let mut seen: HashSet<String> = HashSet::new();

    for combination in nutrient_vec.into_iter().combinations(arguments.daily_meals) {
        let sum = combination.iter().map(|i| i.kcal).sum();
        if combination.iter().all(|combo| !seen.contains(&combo.name))
            && (arguments.kcal_lower_bound..arguments.kcal_upper_bound).contains(&sum)
        {
            combination
                .iter()
                .all(|combo| seen.insert(combo.name.clone()));
            matched_list.push(combination);
            if matched_list.len() == arguments.total_days {
                return matched_list;
            }
        }
    }

    matched_list
}
*/

pub fn match_bounds(nutrients: &[FoodItem], arguments: Arguments) -> FinalMealPlan {
    let mut matched_list: Vec<DayOfMeals> = vec![];
    let mut seen: HashSet<&str> = HashSet::new();

    // iterates through each combination of meals
    for combination in nutrients
        .iter()
        .cloned()
        .combinations(arguments.daily_meals)
    {
        // checks if food has already been used
        if combination.iter().any(|food| seen.contains(food.name)) {
            continue;
        }

        // checks if combination fits within calorie range
        let sum: u16 = combination.iter().map(|i| i.kcal).sum();
        if !(arguments.kcal_lower_bound..arguments.kcal_upper_bound).contains(&sum) {
            continue;
        }

        // marks food as used
        for food in &combination {
            seen.insert(food.name);
        }

        // if all of the previous are successful, add this combination to our list
        matched_list.push(DayOfMeals(combination));

        // return when we reach the amount of days the user requested
        if matched_list.len() == arguments.total_days {
            return FinalMealPlan(matched_list);
        }
    }

    FinalMealPlan(matched_list)
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
    let data_set: Vec<FoodItem> = json::from_str(json_data_set).unwrap();
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
