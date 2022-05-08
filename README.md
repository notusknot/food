# Food

Work in progress - a program that recommends you a weekly meal plan based on your needs

Roadmap/Todo:
> Sorted by estimated difficulty

- Filter based on more than one nutrient (should be pretty easy)
- Rewrite in Rust
- Implement breakfast/lunch/dinner/snack sorting (will take brute force work on database)
- Build web interface and compile Rust to WASM + integrate
- More micronutrient filtering options
- Filtering options based on health focus, condition, or diet (i.e. optimize for muscle gain, to help
with diabetes, vegetarian, etc)
- Content-based recommendation engine (requires database extension) 

The Python version was built for testing and currently works, but not very well. It depends on the itertools, json, and sqlite libraries. 
The Rust version is intended to be used moving forward and is much faster. Currently, it is equally immature and doesn't have the same functionality as the Python one, but is several magnitudes of order faster. 

This project is in a very unstable state and is likely to change a lot over the course of time.

As a quick usage overview, the Rust program takes in 4 parameters: lower bound, upper bound, meals (per day) and total days. An example command to be run could be:
```bash
cargo run 1800 2200 3 7
```
which currently returns:
```rust
[[446, 183, 1387], [446, 183, 1371], [446, 183, 1356], [446, 183, 1308], [446, 183, 1413], [446, 183, 1210], [446, 183, 1444]]
```
The release build takes about 30ms to do everything.
