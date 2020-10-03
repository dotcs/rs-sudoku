use clap::{App, Arg};

mod sudoku;

fn main() {
    let matches = App::new("Rust Sudoku Solver")
        .version("0.0.1")
        .author("dotcs <git@dotcs.me>")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .help("Sets the file to read the Sudoku from")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    let mut s = sudoku::Sudoku::new();
    s.read(matches.value_of("INPUT").unwrap());
    s.solve();
    let unresolved = sudoku::Sudoku::fmt(&s.get_unsolved());
    let solved = sudoku::Sudoku::fmt(&s.grid);
    let solved_iter: Vec<&str> = solved.split("\n").collect();
    for (i, line) in unresolved.split("\n").enumerate() {
        println!("{} -> {}", line, solved_iter.get(i).unwrap());
    }
}
