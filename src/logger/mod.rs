//! Support for logging.

use env_logger::fmt::{Color, Formatter, Target};
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::Write;

/// Represents a logger.
pub struct Logger {
    stderr_logger: env_logger::Logger,
    stdout_logger: env_logger::Logger,
}

impl Logger {
    /// Initializes the global logger.
    pub fn init(level: LevelFilter) {
        let fmt = |buf: &mut Formatter, record: &Record| {
            let mut style = buf.style();

            let level = match &record.level() {
                Level::Error => style.set_bold(true).set_color(Color::Red).value("error: "),
                Level::Warn => style
                    .set_bold(true)
                    .set_color(Color::Yellow)
                    .value("warning: "),
                Level::Info => style.set_bold(true).value(""),
                Level::Debug => style.set_bold(true).value("debug: "),
                Level::Trace => style.set_bold(true).value("trace: "),
            };
            writeln!(
                buf,
                "[{}] {}{}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                record.args()
            )
        };

        let stderr_logger = env_logger::builder()
            .target(Target::Stderr)
            .filter_level(level)
            .format(fmt)
            .build();
        let stdout_logger = env_logger::builder()
            .target(Target::Stdout)
            .filter_level(level)
            .format(fmt)
            .build();

        let logger = Logger {
            stderr_logger,
            stdout_logger,
        };

        // Set the logger
        let r = log::set_boxed_logger(Box::new(logger));
        if r.is_ok() {
            log::set_max_level(level);
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        match metadata.level() {
            Level::Error => self.stderr_logger.enabled(metadata),
            _ => self.stdout_logger.enabled(metadata),
        }
    }

    fn log(&self, record: &Record) {
        match record.metadata().level() {
            Level::Error => self.stderr_logger.log(record),
            _ => self.stdout_logger.log(record),
        }
    }

    fn flush(&self) {}
}
