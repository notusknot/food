pub mod data;

use food_lib::{DayOfMeals, FinalMealPlan};
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example

#[wasm_bindgen]
pub fn match_bounds_js(
    kcal_lower_bound: u16,
    kcal_upper_bound: u16,
    daily_meals: usize,
    total_days: usize,
) -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let arguments = food_lib::CliArgs {
        kcal_lower_bound,
        kcal_upper_bound,
        daily_meals,
        total_days,
    };

    let meal_plan = food_lib::match_bounds(data::NUTRIENTS, arguments).unwrap();
    let result = FinalMealPlan {
        plan: meal_plan
            .into_iter()
            .map(|day| DayOfMeals { day })
            .collect(),
    };

    let body = document.body().expect("document should have a body");
    let val = document.create_element("pre")?;
    val.set_text_content(Some(&format!("{}", result)));

    body.append_child(&val)?;

    Ok(())
}
