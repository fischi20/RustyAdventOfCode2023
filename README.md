# Advent of Code 2023

This is my repository to solve advent of code 2023 problems in rust.

## Requirements

- rust/cargo
- python3

## Usage

### run all days

args:

- nocapture is used to see stderr and stdout for results
- color is used for a nicer print

```bash
cargo test -- --nocapture --color
```

### run a specific day

```bash
cargo test dayX -- --nocapture --color

```

### create a day

```bash
python3 srcripts/create_day.py
```
