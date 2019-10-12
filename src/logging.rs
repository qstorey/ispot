use env_logger;
use log::LevelFilter;

pub fn configure(debug: bool) {
    let mut builder = env_logger::Builder::new();

    builder
        .filter_module("hyper", LevelFilter::Warn)
        .filter_module("tokio_reactor", LevelFilter::Warn)
        .filter_module("reqwest", LevelFilter::Warn);
    if debug {
        builder.filter_level(LevelFilter::Debug);
    } else {
        builder.filter_level(LevelFilter::Info);
    }
    builder.init();
}
