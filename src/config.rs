use clap::{value_t_or_exit, ArgMatches};
use log::info;

pub struct Config {
    pub input_file: String,
    pub max_tries: u32,
    pub show_unsolved: bool,
}

impl Config {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        let input_file = String::from(matches.value_of("INPUT").unwrap());
        info!("Using input file: {}", input_file);
        let max_tries = value_t_or_exit!(matches.value_of("max-tries"), u32);
        info!("Using maximum number of tries: {}", max_tries);
        let show_unsolved = matches.is_present("show-unsolved");

        Config {
            input_file,
            max_tries,
            show_unsolved,
        }
    }
}
