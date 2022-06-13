
<div id="top"></div>

<div align="center">
  <h2 align="center">Food (name is a work in progress)</h2>

  <p align="center">
    A program (and Rust library) that recommends you a meal plan based on your needs
    <br />
    <a href="https://github.com/othneildrew/Best-README-Template">View Demo</a>
    ·
    <a href="https://github.com/notusknot/food/issues">Report Bug</a>
    ·
    <a href="https://github.com/notusknot/food/issues">Request Feature</a>
  </p>
</div>

[![Latest Release][release-badge]][release-latest] [![GPL3 License][license-badge]][license-url] [![LOC][loc-badge]][loc-report] ![Lint Status][lint-badge]


[license-badge]: https://img.shields.io/github/license/notusknot/food.svg
[license-url]: https://github.com/notusknot/food/blob/main/LICENSE
[release-badge]: https://img.shields.io/github/tag/notusknot/food.svg
[release-latest]: https://github.com/notusknot/food/releases/latest
[loc-badge]: https://tokei.rs/b1/github/notusknot/food
[loc-report]: https://github.com/notusknot/food
[lint-badge]: https://github.com/notusknot/food/actions/workflows/test-and-build.yaml/badge.svg

<!-- TABLE OF CONTENTS -->
<h2>Table of Contents</h2>
<ul>
  <li><a href="#about">About</a></li>
  <li>
    <a href="#getting-started">Getting Started</a>
    <ul>
      <li><a href="#prerequisites">Prerequisites</a></li>
      <li><a href="#installation">Installation</a></li>
      <ul>
        <li><a href="#with-nix">With nix</a></li>
        <li><a href="#with-cargo">With cargo</a></li>
      </ul>
    </ul>
  </li>
  <li><a href="#usage">Usage</a></li>
  <li><a href="#performance">Performance</a></li>
  <li><a href="#roadmap">Roadmap</a></li>
  <li><a href="#contributing">Contributing</a></li>
  <li><a href="#license">License</a></li>
  <li><a href="#contact">Contact</a></li>
  <li><a href="#acknowledgments">Acknowledgments</a></li>
</ul>



<!-- ABOUT THE PROJECT -->
## About

This project is my first one done in Rust, so please excuse any code that could be improved. However, I am working hard to make sure this program is fast, safe, and correct, in the spirit of the Rust language.

A changelog can be found in [CHANGELOG.md]

Check out some benchmarks in the [performance](#performance) section!

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

If you would just like to download and run the program, executables for Windows, MacOS and Linux can be found in the [releases](https://github.com/notusknot/food/releases) page. If you would like to compile the program locally on your machine:

### Prerequisites
* cargo & rustc
OR
* nix

### Installation

First, clone the repo:
```
git clone https://github.com/notusknot/food
```
```
cd food
```

#### Using Nix

```
nix build .
```
The binary and library will be in `result/`

#### Using Cargo

To build the cli:
```
cargo build --release
```

To build just the library:
```
cargo build --lib --release
```
The binary and library will be in `target/release/`

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Generate a three meals for one day, between 2100 and 2200 calories:
```
food 2100 2200 3 1
```

Generate a whole week of meals between 2500 and 2600 calories per day, with 4 meals per day:
```
food 2500 2600 4 7
```

Generate a year's worth of meals between 3600 and 3700 calories per day, with 8 meals per day:
```
food 3600 3700 8 365
```

<p align="right">(<a href="#top">back to top</a>)</p>

## Performance

As of the current version, food is very fast. Generating an average meal plan takes about 5 ms:
```
$ hyperfine "target/release/food 1800 2200 3 7" --warmup 30
Benchmark 1: target/release/food 1800 2200 3 7
  Time (mean ± σ):       5.3 ms ±   0.4 ms    [User: 2.2 ms, System: 2.1 ms]
  Range (min … max):     3.8 ms …   7.2 ms    283 runs
```
 The program can also handle generating large amounts of complex plans quickly:
 ```
$ 	hyperfine "target/release/food 3500 3600 8 365" --warmup 30
Benchmark 1: target/release/food 3500 3600 8 365
  Time (mean ± σ):     135.6 ms ±   2.6 ms    [User: 20.1 ms, System: 113.1 ms]
  Range (min … max):   130.0 ms … 141.5 ms    20 runs
```


<!-- ROADMAP -->
## Roadmap

- [ ] Custom recipes to database
- [ ] Family planning: find meal plans that work for a group of people
- [ ] Community-submitted recipes to database
- [ ] Pricing per meal, day, week, nutrient, etc
- [ ] Automatic grocery list
- [ ] Pantry matching (only use ingredients user already has)
- [ ] Sharing/exporting plans
- [ ] Filter based on more than one nutrient (should be pretty easy)
- [ ] Implement breakfast/lunch/dinner/snack sorting (will take brute force work on database)
- [ ] Build web interface and compile Rust to WASM + integrate
- [ ] More micronutrient filtering options
- [ ] Filtering options based on health focus, condition, or diet (i.e. optimize for muscle gain, to help
with diabetes, vegetarian, etc)
- [ ] Content-based recommendation engine (requires database extension)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

This project is still in a very rough phase, so contributions are greatly apreciated. Thank you!

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the GPL License. See `LICENSE` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

My website, containing all my up-to-date contact info - [notusknot.com](https://notusknot.com)

If my website ever goes down, it is archived on the [Wayback Machine](https://web.archive.org/web/*/notusknot.com)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [The Rust Discord server and everyone on it](https://discord.gg/rust-lang-community) for helping me learn Rust
* [The Rust Book](https://doc.rust-lang.org/stable/book/) for helping me learn Rust
* [Choose an Open Source License](https://choosealicense.com) for helping me choose GPL
* [Best README template](https://github.com/othneildrew/Best-README-Template) for the base of this very README

<p align="right">(<a href="#top">back to top</a>)</p>