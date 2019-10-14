use env_logger;
use log::LevelFilter;

pub fn configure(verbosity: u64) {
    let mut builder = env_logger::Builder::new();

    builder
        .filter_module("hyper", LevelFilter::Warn)
        .filter_module("tokio_reactor", LevelFilter::Warn)
        .filter_module("reqwest", LevelFilter::Warn);
    match verbosity {
        0 => builder.filter_level(LevelFilter::Info),
        1 => builder.filter_level(LevelFilter::Debug),
        2 | _ => builder.filter_level(LevelFilter::Trace),
    };

    builder.init();
}
