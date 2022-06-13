# Food

Work in progress - a program that recommends you a meal plan based on your needs

If you have Nix installed and would like to try it out: 
```
nix run github:notusknot/food 0 2000 3 7
```

## Building

First, clone the repo:
```
git clone https://github.com/notusknot/food
```
```
cd food
```

### Using Nix

```
nix build .
```
The binary and library will be in `result/`

### Using Cargo

To build the cli:
```
cargo build --release
```

To build just the library:
```
cargo build --lib --release
```
The binary and library will be in `target/release/`

Roadmap/Todo:

- Custom recipes to database
- Family planning: find meal plans that work for a group of people
- Community-submitted recipes to database
- Pricing per meal, day, week, nutrient, etc
- Automatic grocery list
- Pantry matching (only use ingredients user already has)
- Sharing/exporting plans
- Filter based on more than one nutrient (should be pretty easy)
- Implement breakfast/lunch/dinner/snack sorting (will take brute force work on database)
- Build web interface and compile Rust to WASM + integrate
- More micronutrient filtering options
- Filtering options based on health focus, condition, or diet (i.e. optimize for muscle gain, to help
with diabetes, vegetarian, etc)
- Content-based recommendation engine (requires database extension) 

The Rust version is intended to be used moving forward and is much faster. Currently, it is equally immature and doesn't have the same functionality as the Python one, but is several magnitudes of order faster. 

This project is in a very unstable state and is likely to change a lot over the course of time.

As a quick usage overview, the Rust program takes in 4 parameters: lower bound, upper bound, meals (per day) and total days. An example command to be run could be:
```bash
target/release/food 1800 2200 3 7
```
which currently returns:
```
New Day:
────────────────────────────────────────────
name: Lighter Chinese chilli beef, 
ingredients: ¼ tsp chilli flakes, 
instructions: Pour the remaining oil into the wok and heat it again until very hot. Throw in the garlic, ginger, red pepper and spring onions, and stir-fry for 2-3 mins until starting to brown. Add the chilli flakes, then pour in the soy sauce and orange juice mix with 4-5 tbsp water. As it comes to the boil, stir in the beef and steamed veg, and cook briefly just to heat through. If you want a little more sauciness, splash in another 1-2 tbsp water., 
servings: 2, 
kcal: 389, 
fat: 16, 
carbs: 26, 
protein: 33

name: Spinach, cheese & onion rice torte, 
ingredients: 2 tsp sesame seed, 
instructions: Place the tin on the preheated baking tray and bake for 10 mins, then reduce the temperature to 160C/140C fan/gas 4 and bake for a further 30 mins until the pastry is golden. Leave to cool for 10 mins before cutting into squares., 
servings: 6, 
kcal: 631, 
fat: 38, 
carbs: 55, 
protein: 18

name: Butternut macaroni cheese, 
ingredients: 50g parmesan, 
instructions: Take the sauce off the heat and mash in a third of the squash with the cheddar and half the Parmesan. Season, then stir in the drained macaroni with the remaining squash. Tip into an ovenproof dish, scatter with the remaining Parmesan and bake for 15 mins until golden and bubbling., 
servings: 4, 
kcal: 828, 
fat: 38, 
carbs: 94, 
protein: 35

...

New Day:
────────────────────────────────────────────
name: Easy sausage casserole, 
ingredients: 4 slices  ciabatta, 
instructions: Meanwhile, toast the ciabatta, then rub with the remaining garlic, drizzle with oil, scatter with a few thyme leaves and serve with the casserole., 
servings: 2, 
kcal: 701, 
fat: 34, 
carbs: 71, 
protein: 29

name: Sicilian potato cake, 
ingredients: handful fresh  thyme leaves, 
instructions: Bake for about 1hr 10 mins until the potato cake is set, with a slight wobble in the middle. Let it rest for 5 mins then loosen from the sides with a knife before releasing the tin. Slide onto a plate and sprinkle with the thyme leaves. Serve hot or warm. Can be prepared up to the end of step 3 the night before and kept covered in the fridge., 
servings: 12, 
kcal: 381, 
fat: 24, 
carbs: 22, 
protein: 20

name: Roast rack of lamb with Moroccan spices, 
ingredients: Greek yogurt, 
instructions: Put the lamb on a warmed plate and leave it to rest for 5 minutes. Slice it in half to give 3-4 cutlets each, then cut in half again. Spoon the couscous on to two plates, scatter over the almonds and top with the lamb. Serve with a spoonful of Greek yogurt., 
servings: 2, 
kcal: 888, 
fat: 67, 
carbs: 40, 
protein: 34
```

The release build takes about 6ms to run on my machine.

```bash
$ hyperfine "target/release/food 1800 2200 3 7" --warmup 100
Benchmark 1: target/release/food 1800 2200 3 7
  Time (mean ± σ):       6.2 ms ±   0.5 ms    [User: 2.5 ms, System: 2.5 ms]
  Range (min … max):     5.4 ms …   7.9 ms    273 runs
```
