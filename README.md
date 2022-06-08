# Food

Work in progress - a program that recommends you a meal plan based on your needs

If you have Nix installed and would like to try it out: 
```nix run github:notusknot/food 0 2000 3 7```
Note: this currently doesn't work because it requires the database to be present. See more on building locally below:

## Building

First, clone the repo:
```git clone https://github.com/notusknot/food```
```cd food```

### Using Nix

```nix build .```
The binary and library will be in `result/`

### Using Cargo

To build the cli:
```cargo build```

To build just the library:
```cargo build --no-default-features```

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
```rust
[[("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Chinese roast duck", 1387)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Sugar-dusted snowflake cake", 1371)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Beef & beer pie", 1356)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Steamed vanilla sponge with butterscotch sauce & custard", 1308)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Roast guinea fowl with chestnut, sage & lemon stuffing", 1413)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Beef, potato & banana curry with cashew rice", 1210)], [("Cauliflower & macaroni cheese", 446), ("Bean & pesto mash", 183), ("Caramelised white chocolate, ginger caramel & macadamia tarts", 1444)]]
```

The release build takes 35ms to run on my machine.

```bash
time target/release/food 1800 2200 3 7
[[("Cauliflower & macaroni cheese", 19), ("Bean & pesto mash", 11), ("Chinese roast duck", 58)], [("Cauliflower & macaroni cheese", 19), ("Bean & pesto mash", 11), ("Sugar-dusted snowflake cake", 12)], [("Cauliflower & macaroni cheese", 19), ("Bean & pesto mash", 11), ("Beef & beer pie", 56)], [("Cauliflower & macaroni cheese", 19), ("Bean & pesto mash", 11), ("Steamed vanilla sponge with butterscotch sauce & custard", 18)], [("Cauliflower & macaroni cheese", 19), ("Bean & pesto mash", 11), ("Roast guinea fowl with chestnut, sage & lemon stuffing", 105)], [("Cauliflower & macaroni cheese", 19), ("Bean & pesto mash", 11), ("Beef, potato & banana curry with cashew rice", 57)], [("Cauliflower & macaroni cheese", 19), ("Bean & pesto mash", 11), ("Caramelised white chocolate, ginger caramel & macadamia tarts", 16)]]
target/release/food 1800 2200 3 7  0.02s user 0.01s system 99% cpu 0.034 total
```
