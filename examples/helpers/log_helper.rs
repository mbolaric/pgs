use log::LevelFilter;
use log4rs::{append::console::ConsoleAppender, config::{Appender, Root}, Config};

pub fn init_logging() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}