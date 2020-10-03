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
-----------
826|195|347
374|682|915
951|743|628
-----------
519|326|874
248|957|136
763|418|259
```

To compare this with the input use the `--show-unresolved` flag.

```
$ cargo run -- examples/sudoku1.txt --show-unsolved
Using input file: examples/sudoku1.txt
Solved. Needed 63 tries.
xxx|26x|7x1 -> 435|269|781
68x|x7x|x9x -> 682|571|493
19x|xx4|5xx -> 197|834|562
----------- -> -----------
82x|1xx|x4x -> 826|195|347
xx4|6x2|9xx -> 374|682|915
x5x|xx3|x28 -> 951|743|628
----------- -> -----------
xx9|3xx|x74 -> 519|326|874
x4x|x5x|x36 -> 248|957|136
7x3|x18|xxx -> 763|418|259
```

In the `examples` folder a few Sudokus are located that can be use to quickly try out this Rust implementation.
Other Sudokus can be used as well.
The files should match the [same format](./examples/sudoku1.txt) as shown in the examples.
