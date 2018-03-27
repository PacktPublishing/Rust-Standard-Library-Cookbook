#[macro_use]
extern crate log;

use log::{Level, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::{error, fmt, result};
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

// This logger will write logs into a file on disk
struct FileLogger {
    level: Level,
    writer: RwLock<BufWriter<File>>,
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Check if the logger is enabled for a certain log level
        // Here, you could also add own custom filtering based on targets or regex
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self.writer
                .write()
                .expect("Failed to unlock log file writer in write mode");
            let now = SystemTime::now();
            let timestamp = now.duration_since(UNIX_EPOCH).expect(
                "Failed to generate timestamp: This system is operating before the unix epoch",
            );
            // Write the log into the buffer
            write!(
                writer,
                "{} {} at {}: {}\n",
                record.level(),
                timestamp.as_secs(),
                record.target(),
                record.args()
            ).expect("Failed to log to file");
        }
        self.flush();
    }

    fn flush(&self) {
        // Write the buffered logs to disk
        self.writer
            .write()
            .expect("Failed to unlock log file writer in write mode")
            .flush()
            .expect("Failed to flush log file writer");
    }
}

impl FileLogger {
    // A convenience method to set everything up nicely
    fn init(level: Level, file_name: &str) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)?;
        let writer = RwLock::new(BufWriter::new(file));
        let logger = FileLogger { level, writer };
        // set the global level filter that log uses to optimize ignored logs
        log::set_max_level(level.to_level_filter());
        // set this logger as the one used by the log macros
        log::set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}

// Our custom error for our FileLogger
#[derive(Debug)]
enum FileLoggerError {
    Io(io::Error),
    SetLogger(log::SetLoggerError),
}

type Result<T> = result::Result<T, FileLoggerError>;
impl error::Error for FileLoggerError {
    fn description(&self) -> &str {
        match *self {
            FileLoggerError::Io(ref err) => err.description(),
            FileLoggerError::SetLogger(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            FileLoggerError::Io(ref err) => Some(err),
            FileLoggerError::SetLogger(ref err) => Some(err),
        }
    }
}

impl fmt::Display for FileLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileLoggerError::Io(ref err) => write!(f, "IO error: {}", err),
            FileLoggerError::SetLogger(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<io::Error> for FileLoggerError {
    fn from(err: io::Error) -> FileLoggerError {
        FileLoggerError::Io(err)
    }
}

impl From<log::SetLoggerError> for FileLoggerError {
    fn from(err: log::SetLoggerError) -> FileLoggerError {
        FileLoggerError::SetLogger(err)
    }
}

fn main() {
    FileLogger::init(Level::Info, "log.txt").expect("Failed to init FileLogger");
    trace!("Beginning the operation");
    info!("A lightning strikes a body");
    warn!("It's moving");
    error!("It's alive!");
    debug!("Dr. Frankenstein now knows how it feels to be god");
    trace!("End of the operation");
}
