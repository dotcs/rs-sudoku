use clap::{value_t_or_exit, App, Arg};
use log::{debug, error, info, LevelFilter};
use std::process;

mod logger;
mod sudoku;

use sudoku::solver::{Backtracing, Montecarlo, Solver};

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
            Arg::with_name("max-tries")
                .long("max-tries")
                .required(false)
                .default_value("100000")
                .help("Defines the maximum number of tries to iteratively solve the sudoku."),
        )
        .arg(
            Arg::with_name("algorithm")
                .long("algorithm")
                .possible_values(&["backtracing", "montecarlo"])
                .default_value("backtracing")
                .help("Selects which algorithm will be used to solve the sudoku."),
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

    let input_file = matches.value_of("INPUT").unwrap();
    let max_tries = value_t_or_exit!(matches.value_of("max-tries"), u32);
    let show_unresolved = matches.is_present("show-unsolved");

    info!("Using input file: {}", input_file);
    info!("Using maximum number of tries: {}", max_tries);

    let mut s = sudoku::Sudoku::new();

    s.read(input_file);

    let mut solver = match matches.value_of("algorithm") {
        Some("backtracing") => Box::new(Backtracing::new(max_tries)) as Box<dyn Solver>,
        Some("montecarlo") => Box::new(Montecarlo::new(max_tries, 0.15)) as Box<dyn Solver>,
        _ => Box::new(Backtracing::new(max_tries)) as Box<dyn Solver>,
    };

    s = solver.solve(s);

    match solver.is_success() {
        true => {
            info!(
                "Success. Solving the sudoku needed {} tries.",
                solver.get_tries()
            );
            s.print(show_unresolved);
            process::exit(0);
        }
        false => {
            error!(
                "Fatal. Exceeded the limit of {} tries. \
                Make sure that the sudoku is valid and consider increasing this \
                number with the --max-tries argument.",
                max_tries
            );
            process::exit(1);
        }
    }
}
