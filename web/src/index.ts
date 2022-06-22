//@ts-ignore
import init, { match_bounds_js } from 'food-web';
// Don't worry if vscode told you can't find my-crate
// It's because you're using a local crate
// after yarn dev, wasm-pack plugin will install my-crate for you

function getInputValue() {
  // Selecting the input element and get its value 
  let kcal_lower_bound: number = +(document.getElementById("kcal_lb") as HTMLInputElement).value;
  let kcal_upper_bound: number = +(document.getElementById("kcal_ub") as HTMLInputElement).value;
  let daily_meals: number = +(document.getElementById("daily_meals") as HTMLInputElement).value;
  let total_days: number = +(document.getElementById("total_days") as HTMLInputElement).value;

  match_bounds_js(kcal_lower_bound, kcal_upper_bound, daily_meals, total_days)
}

init().then(() => {
  console.log('init wasm-pack');

  let btn: HTMLElement = document.getElementById("gen_btn") as HTMLElement;
	// add event listener for the button, for action "click"
	btn.addEventListener("click", getInputValue);
});