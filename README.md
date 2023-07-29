# 2017 Advent of Code (Rust)

This repository implements solutions to the puzzles in the [2017 Advent of Code](https://adventofcode.com/2017) using Rust.


## Preface

This was a vehicle to learn Rust, so I presume not everything done here will be deemed idiomatic by Rust specialists.

Generally speaking, the solutions are organised predominantly for comprehension. They strive to arrive at an answer in a reasonable period of time, but they typically prioritise optimal understanding over optimal performance.

The examples are representative of my thinking and coding style.


## Getting Started

### Prerequisites

The project requires `rust 1.71.0`, but any reasonably current version of Rust will likely work.  I tend to code done the middle of any language specification.

If you use a Golang manager that responds to `.tool-versions`, you should be switched to `1.71.0` automatically. I recommend [ASDF](https://github.com/asdf-vm/asdf) for those on platforms that support it.

### Installation

Dependencies are tracked in the `Cargo.toml`.  They should be downloaded automatically the first time
you run the main program or the tests.

### File Structure

- [data](./data): Puzzle input organised by day
- [site](./site): The calendar and puzzle descriptions
- [src](./src):   Daily solutions and corresponding tests


### Running Daily Solutions

Modify `src/main.rs` to import the daily solution of your choice.

Then invoke the following command in your terminal from the project root.

```
$ cargo run
```

### Running Tests

The only tests are a set of checks to verify solved puzzles.

I often refactor my solutions for clarity (or as I learn new
techniques in subsequent puzzles), so it is helpful to have
these simple tests to give my refactors some confidence.

In Rust, unit tests are placed in the source files, so you
can find all the regression tests in the corresponding mod.rs
files.

To execute the tests, simply execute the following command in
your terminal from the project root.

```
$ cargo test
```