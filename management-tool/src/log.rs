use chrono::Local;
use colored::*;
use log::{Level, LevelFilter};

fn logger_icon<'a>(debug: &'a str, release: &'a str) -> &'a str {
    if !cfg!(feature = "debug") {
        debug
    } else {
        release
    }
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{time} {level} {target} {arrow} {message}",
                time = Local::now()
                    .format("%F\t%T%.3f%Z\t")
                    .to_string()
                    .bright_black(),
                level = {
                    let (color, icon, label) = match record.level() {
                        Level::Info => (Color::Blue, logger_icon("i", "I"), "info"),
                        Level::Warn => (Color::Yellow, logger_icon("⚠", "W"), "warning"),
                        Level::Error => (Color::Red, logger_icon("✖", "E"), "error"),
                        Level::Debug => (Color::Blue, logger_icon("●", "D"), "debug"),
                        Level::Trace => (Color::Magenta, logger_icon("…", "T"), "trace"),
                    };
                    format!(
                        "{} {}{} ",
                        icon.color(color),
                        label.underline().color(color),
                        " ".repeat(7 - label.len())
                    )
                },
                target = {
                    let target: Vec<&str> = record.target().split(":").collect();
                    format!("{: <10} ", target.last().unwrap().bright_black())
                },
                arrow = logger_icon("›", "->").bright_black(),
                message = message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
