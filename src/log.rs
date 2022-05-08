/*!
Before running the following example, ensure to set
the `SERIALIZE_LOG_LEVEL` environment variable to "info"...

Example:

```no_run,shell
$ export SERIALIZE_LOG_LEVEL="info"
```
*/

#![allow(unused_imports)]
#![allow(unused_must_use)]

use log::{trace, debug, info, warn, error, LevelFilter, set_logger};

use log::{
    set_max_level,
    set_boxed_logger
};

use log::{
    Log,
    Record,
    Metadata,
    SetLoggerError
}; use colored::{ Colorize, ColoredString };

use env_logger::filter::{ Builder, Filter };

use std::io::Write;
use std::io::stdout;
use std::ops::Deref;

const FILTER_ENV: &str = "SERIALIZE_LOG_LEVEL";
const PACKAGE_NAME: Option<&'static str> = std::option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = std::option_env!("CARGO_PKG_VERSION");

fn program() -> ColoredString {
    let program = PACKAGE_NAME.expect("Error Unwrapping Package Name");
    let string = &title(program).to_string();
    return string.bold();
}

fn title(s: &str) -> String {
    let normalization = format!("{}", s).to_lowercase();
    let mut array = normalization.chars();
    return array
        .next()
        .map(|first_letter| first_letter.to_uppercase())
        .into_iter()
        .flatten()
        .chain(array)
        .collect()
}

fn line(record: &log::Record) -> ColoredString {
    let line = record.line().expect("Error Unwrapping Line Number");
    let string: String = line.to_string();
    return string.cyan().bold();
}

fn file(record: &log::Record) -> ColoredString {
    let file = record.file().expect("Error Unwrapping File Name");
    return file.italic().underline();
}

fn level(record: &log::Record) -> ColoredString {
    let level = record.level().as_str();
    return match level {
        _ => {
            if      level == "TRACE"    { "Trace".bright_magenta()  }
            else if level == "DEBUG"    { "Debug".bright_green()    }
            else if level == "INFO"     { "Normal".bright_blue()    }
            else if level == "WARN"     { "Warning".bright_yellow() }
            else if level == "ERROR"    { "Error".bright_red()      }
            else                        { "".clear()                }
        }
    };
}

/// impl ... {
///     fn enabled(&self, metadata: &Metadata) -> bool {
///         true
///     }
///
///     fn log(&self, record: &log::Record) {
///
///         print!("[{}]", title(&program()).bold());
///         println!("[{}]: {}", level(record), record.args());
///         print!("    {} ({}), {} Line {}", "﬌".magenta(), file(record), "@".blue(), line(record).underline());
///         print!("{} {}", "\n", "\n");
///
///         stdout().flush();
///
///     }
///
///     fn flush(&self) {}
/// }

fn init() {
    std::env::set_var("SERIALIZE_LOG_LEVEL", "debug");
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();
}

pub fn import() {
    init();

    let version = VERSION.unwrap();

    debug!("{}: {}", "Version".dimmed(), version.bold());
}

pub fn version() -> &'static str {
    return VERSION.expect("Error Parsing Version");
}

pub fn name() -> &'static str {
    return PACKAGE_NAME.expect("Error Parsing Package Name");
}

fn main() {
    import();
}