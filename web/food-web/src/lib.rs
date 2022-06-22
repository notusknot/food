pub mod data;

use food_lib::{DayOfMeals, FinalMealPlan};
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let arguments = food_lib::CliArgs {
        kcal_lower_bound: 1800,
        kcal_upper_bound: 1900,
        daily_meals: 3,
        total_days: 7,
    };

    let meal_plan = food_lib::match_bounds(data::NUTRIENTS, arguments).unwrap();
    let result = FinalMealPlan {
        plan: meal_plan
            .into_iter()
            .map(|day| DayOfMeals { day })
            .collect(),
    };

    let val = document.create_element("p")?;
    val.set_text_content(Some(&format!("{}", result)));

    body.append_child(&val)?;

    Ok(())
}
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe {
        alert(&format!("Hello,{}!", name));
    }
}