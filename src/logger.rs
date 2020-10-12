use log::debug;
use log::{LevelFilter, Metadata, Record, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        // empty on purpose
    }
}

static LOGGER: SimpleLogger = SimpleLogger;

fn _init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}

pub fn init(level: u8) {
    let log_level: LevelFilter = match level {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 | _ => LevelFilter::Trace,
    };
    let _ = _init(log_level);
    debug!("Set logging level to: {}", log_level);
}
