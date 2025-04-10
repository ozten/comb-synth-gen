# Combinatorial Synthetic Generator (comb-synth-gen)

A Rust CLI tool for generating combinatorial permutations from template files.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
comb-synth-gen <input_file>
```

Where `<input_file>` is a template file with `NNN` in the filename (e.g., `NNN.frag` or `NNN_math.js`).

## Template Syntax

The tool looks for `EACH<...>` blocks in the input file, where the content between the angle brackets is a comma-separated list of values. For example:

```javascript
var a = EACH<1, 2, 3>;
var b = EACH<3.14, 1.81>;
console.log(a + b);
```

This will generate all possible combinations of the values, creating separate output files for each combination.

## Output

For each combination, the tool will:
1. Create an output file with a zero-padded index (e.g., `000_math.js`, `001_math.js`, etc.)
2. Log the mapping in `combinations.log`

## Example

Given the input file `NNN_math.js` with the content:
```javascript
var a = EACH<1, 2, 3>;
var b = EACH<3.14, 1.81>;
console.log(a + b);
```

Running:
```bash
comb-synth-gen NNN_math.js
```

Will generate:
- `000_math.js` with `var a = 1; var b = 3.14; console.log(a + b);`
- `001_math.js` with `var a = 1; var b = 1.81; console.log(a + b);`
- `002_math.js` with `var a = 2; var b = 3.14; console.log(a + b);`
- `003_math.js` with `var a = 2; var b = 1.81; console.log(a + b);`
- `004_math.js` with `var a = 3; var b = 3.14; console.log(a + b);`
- `005_math.js` with `var a = 3; var b = 1.81; console.log(a + b);`

And a `combinations.log` file containing the mapping of each combination. 

== Development

    RUST_BACKTRACE=1 cargo run --release -- NNN_math.js