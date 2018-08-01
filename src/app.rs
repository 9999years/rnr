#![allow(unknown_lints)]
use clap::{App, Arg};
use std::ffi::{OsStr, OsString};

/// Create application using clap. It sets all options and command-line help.
pub fn create_app<'a>() -> App<'a, 'a> {
    App::new("rnr")
        .version("0.1.6")
        .author("Ismael González <ismgonval@gmail.com>")
        .about("\nrnr is simple file renamer written in Rust.")
        .arg(
            Arg::with_name("EXPRESSION")
                .help("Expression to match (can be a regex)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("REPLACEMENT")
                .help("Expression replacement")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("FILE(S)")
                .help("Target files")
                .required_unless("recursive")
                .conflicts_with("recursive")
                .multiple(true),
        )
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .short("n")
                .help("Only show what would be done (default mode)")
                .conflicts_with("force"),
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .short("f")
                .help("Make actual changes to files"),
        )
        .arg(
            Arg::with_name("backup")
                .long("backup")
                .short("b")
                .help("Generate file backups before renaming"),
        )
        .arg(
            Arg::with_name("include-dirs")
                .long("include-dirs")
                .short("D")
                .help("Rename matching directories"),
        )
        .arg(
            Arg::with_name("recursive")
                .long("recursive")
                .short("r")
                .value_name("PATH")
                .help("Recursive mode"),
        )
        .arg(
            Arg::with_name("max-depth")
                .requires("recursive")
                .long("max-depth")
                .short("d")
                .takes_value(true)
                .value_name("LEVEL")
                .validator(is_integer)
                .help("Set max depth in recursive mode"),
        )
        .arg(
            Arg::with_name("hidden")
                .requires("recursive")
                .long("hidden")
                .short("-x")
                .help("Include hidden files and directories"),
        )
        .arg(
            Arg::with_name("silent")
                .long("silent")
                .short("s")
                .help("Do not print any information"),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .possible_values(&["always", "auto", "never"])
                .default_value("auto")
                .help("Set color output mode"),
        )
}

#[allow(clippy)]
/// Check if the input provided is valid unsigned integer
fn is_integer(arg_value: String) -> Result<(), String> {
    match arg_value.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Value provided is not an integer".to_string()),
    }
}

#[cfg(test)]
mod test {}
