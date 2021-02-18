use lib_a::*;
use lib_b::logging_fib;

use log::{info, Level, LevelFilter, Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();
    let p = Person {
        name: "John Smith".to_string(),
        age: 42,
        phones: vec!["123".to_string(), "456".to_string(), "7890".to_string()],
    };
    let s = serialize_person(&p).unwrap();
    info!("{}", s);
    logging_fib(42);
}
