use clap::{App, Arg};

pub fn configure_parser() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
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
}
