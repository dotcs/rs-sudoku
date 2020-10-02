# Simple Sudoku Solver written in Rust

This project is a toy project that I have written to get familiar with the Rust language.
It contains a simple algorithm to solve Sudokus.

## Getting started

Install [Rustup](https://rustup.rs/) first.
Then clone this repository and run

```console
$ cargo run -- examples/sudoku1.txt
Using input file: examples/sudoku1.txt
Solved. Needed 63 tries.
435|269|781
682|571|493
197|834|562
--
826|195|347
374|682|915
951|743|628
--
519|326|874
248|957|136
763|418|259
```

In the `examples` folder a few Sudokus are located that can be use to quickly try out this Rust implementation.
Other Sudokus can be used as well.
The files should match the [same format](./examples/sudoku1.txt) as shown in the examples.
