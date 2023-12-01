# Advent of Code 2023

This is my repository to solve advent of code 2023 problems in rust.
I won't try to find the most optimal solution, so please don't use it as a reference,
there for sure are cleaner and more efficient solutions out there

## Template

The [template](https://github.com/fischi20/RustyAdventOfCode2023/tree/template) branch contains a template without any completed days.

## Requirements

- rust/cargo
- python3

## Usage

### run all days

args:

- nocapture is used to see stderr and stdout for results
- color is used for a nicer print

```bash
cargo test -- --nocapture --color always
```

### run a specific day

```bash
cargo test dayX -- --nocapture --color always

```

### create a day

```bash
python3 srcripts/create_day.py
```
