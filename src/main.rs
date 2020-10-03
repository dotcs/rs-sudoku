use clap::{App, Arg};
use log::{debug, info, LevelFilter};

mod logger;
mod sudoku;

fn main() {
    let matches = App::new("Rust Sudoku Solver")
        .version("0.2.0")
        .about("Simple sudoku solver written in Rust")
        .author("dotcs <git@dotcs.me>")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .help("Sets the file to read the sudoku from")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("show-unsolved")
                .long("show-unsolved")
                .required(false)
                .help("Shows the unsolved sudoku next to the solution"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help(
                    "Sets the level of verbosity, can be used multiple times to increase verbosity",
                ),
        )
        .get_matches();

    // Configure logger as early as possible.
    let log_level: LevelFilter = match matches.occurrences_of("verbosity") {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 | _ => LevelFilter::Trace,
    };
    let _ = logger::init(log_level);

    debug!("Set logging level to: {}", log_level);
    info!("Using input file: {}", matches.value_of("INPUT").unwrap());

    let mut s = sudoku::Sudoku::new();
    s.read(matches.value_of("INPUT").unwrap());
    s.solve();
    s.print(matches.is_present("show-unsolved"));
}
