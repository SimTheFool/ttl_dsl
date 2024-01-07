pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::trace;
pub use log::warn;
pub use log::Log;
pub use log::{Level, LevelFilter, Metadata, MetadataBuilder, Record, RecordBuilder};

static INITIALIZED: std::sync::Once = std::sync::Once::new();

pub fn set_static_logger(logger: Box<impl log::Log + 'static>) {
    INITIALIZED.call_once(|| {
        log::set_boxed_logger(logger).unwrap();
    });
}
pub fn set_static_logger_level(level: log::LevelFilter) {
    log::set_max_level(level);
}
