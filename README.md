# Governor

A small Rust-powered utility that converts design token in form of a YAML file. Currently has 2 mode of translations:

- YAML to JSON.
- JSON to SCSS

Still largely WIP, but so far it's usable for my daily use.

## Using The Project

1. Install Rust, instructions [here](https://www.rust-lang.org/learn/get-started)
2. Clone this project
3. Store the design token file you want to translate in the root of the project
4. Run the project with `cargo run` followed by the type of file to translate, json or yaml. For example:

- To translate YAML to JSON `cargo run yaml`
- To translate JSON to SCSS `cargo run json`

## Related Project

- [Governor Figma](https://github.com/lunchboxav/governor-figma) - Generates JS file from JSON gathered from Figma API
- [Governor Generator](https://github.com/lunchboxav/governor-generator) - Generates React component from JSON gathered from Figma API
