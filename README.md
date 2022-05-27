# Food

Work in progress - a program that recommends you a weekly meal plan based on your needs

Roadmap/Todo:

- Custom recipes to database
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
cargo run 1800 2200 3 7
```
which currently returns:
```rust
[[("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Brandy pudding", 764)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Crunchy cauliflower, apple & blue cheese salad", 417)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Sticky glazed ribs", 604)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Egg fried rice with prawns", 401)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Full English salad", 508)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Celery sticks with blue cheese dip", 158)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Haricot bean & truffle mash", 432)]]
```
The release build takes about 30ms to do everything.
