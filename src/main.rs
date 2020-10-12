use log::{error, info};
use std::process;

mod cli;
mod config;
mod logger;
mod sudoku;

use config::Config;
use sudoku::solver::{Backtracing, Montecarlo, Solver};

fn main() {
    let parser = cli::configure_parser().version("0.3.0");
    let matches = parser.get_matches();

    // Configure logger as early as possible.
    logger::init(matches.occurrences_of("verbosity") as u8);

    let cfg = Config::from_matches(&matches);
    let mut s = sudoku::Sudoku::new();
    s.read(&cfg.input_file);

    let mut solver = match matches.value_of("algorithm") {
        Some("backtracing") => Box::new(Backtracing::new(cfg.max_tries)) as Box<dyn Solver>,
        Some("montecarlo") => Box::new(Montecarlo::new(cfg.max_tries, 0.15)) as Box<dyn Solver>,
        _ => Box::new(Backtracing::new(cfg.max_tries)) as Box<dyn Solver>,
    };

    s = solver.solve(s);

    match solver.is_success() {
        true => {
            info!(
                "Success. Solving the sudoku needed {} tries.",
                solver.get_tries()
            );
            s.print(cfg.show_unsolved);
            process::exit(0);
        }
        false => {
            error!(
                "Fatal. Exceeded the limit of {} tries. \
                Make sure that the sudoku is valid and consider increasing this \
                number with the --max-tries argument.",
                cfg.max_tries
            );
            process::exit(1);
        }
    }
}
