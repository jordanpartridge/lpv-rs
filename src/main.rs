//! lpv-rs: a fast Rust port of Raphael Stolt's lean-package-validator.

mod agent;
mod analyser;
mod archive;
mod cli;
mod commands;
mod diff;
mod error;
mod finder;
mod glob;
mod output;
mod php;
mod presets;

use std::io::Write;
use std::process::ExitCode;

use clap::Parser;
use clap::error::{ContextKind, ContextValue, ErrorKind};

use cli::{Cli, Command, Format};
use commands::Ctx;
use error::LpvError;
use finder::Lookup;
use output::{Output, Verbosity, render_error_block};

fn eprint(text: &str) {
    let mut err = std::io::stderr().lock();
    let _ = err.write_all(text.as_bytes());
    let _ = err.flush();
}

/// The lpv command named on the command line (after resolving Symfony-style
/// unambiguous abbreviations), if any.
fn command_token(args: &[String]) -> Option<(usize, String)> {
    args.iter()
        .enumerate()
        .skip(1)
        .find(|(_, a)| !a.starts_with('-'))
        .map(|(i, a)| (i, a.clone()))
}

/// Symfony's `ArgvInput::hasParameterOption($values, onlyParams: true)`.
fn has_parameter_option(args: &[String], values: &[&str]) -> bool {
    for token in args.iter().skip(1) {
        if token == "--" {
            return false;
        }
        for value in values {
            let leading = if value.starts_with("--") {
                format!("{value}=")
            } else {
                (*value).to_string()
            };
            if token == value || token.starts_with(&leading) {
                return true;
            }
        }
    }
    false
}

/// Whether Symfony keeps the input interactive (`configureIO()`).
fn is_interactive(args: &[String]) -> bool {
    let shell_verbosity = std::env::var("SHELL_VERBOSITY")
        .ok()
        .map(|v| output::php_int(v.trim()));
    !(has_parameter_option(
        args,
        &["--quiet", "-q", "--silent", "--no-interaction", "-n"],
    ) || shell_verbosity.is_some_and(|v| v < 0))
}

/// Symfony's "Do you want to run X instead?" confirmation, written to stdout.
/// Returns whether the user confirmed.
fn confirm_alternative(name: &str, alternative: &str) -> bool {
    let message = format!("Command \"{name}\" is not defined.");
    let pad = " ".repeat(message.chars().count() + 4);
    let prompt = format!(
        "\n{pad}\n  {message}  \n{pad}\n\n Do you want to run \"{alternative}\" instead?  (yes/no) [no]:\n > "
    );
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(prompt.as_bytes());
    let _ = stdout.flush();
    let mut answer = String::new();
    match std::io::stdin().read_line(&mut answer) {
        Ok(0) | Err(_) => false,
        Ok(_) => {
            let _ = stdout.write_all(b"\n");
            let _ = stdout.flush();
            answer
                .trim_start()
                .chars()
                .next()
                .is_some_and(|c| c.eq_ignore_ascii_case(&'y'))
        }
    }
}

fn clap_error_message(err: &clap::Error, command: Option<&str>) -> String {
    let invalid = match err.get(ContextKind::InvalidArg) {
        Some(ContextValue::String(s)) => s.clone(),
        Some(ContextValue::Strings(v)) => v.first().cloned().unwrap_or_default(),
        _ => String::new(),
    };
    let flag = invalid
        .split(['=', ' '])
        .next()
        .unwrap_or_default()
        .to_string();
    match err.kind() {
        ErrorKind::UnknownArgument if invalid.starts_with('-') => {
            format!("The \"{flag}\" option does not exist.")
        }
        ErrorKind::UnknownArgument => match command {
            Some(cmd) => format!(
                "Too many arguments to \"{cmd}\" command, expected arguments \"directory\"."
            ),
            None => "No arguments expected.".to_string(),
        },
        ErrorKind::InvalidValue | ErrorKind::NoEquals if !flag.is_empty() => {
            if invalid.contains('<') {
                format!("The \"{flag}\" option requires a value.")
            } else {
                format!("The \"{flag}\" option does not accept a value.")
            }
        }
        ErrorKind::TooManyValues if !flag.is_empty() => {
            format!("The \"{flag}\" option does not accept a value.")
        }
        _ => err
            .to_string()
            .lines()
            .next()
            .unwrap_or("Invalid input.")
            .trim_start_matches("error: ")
            .to_string(),
    }
}

fn run() -> i32 {
    let mut args: Vec<String> = std::env::args().collect();

    let mut command_name: Option<&'static str> = None;
    if let Some((idx, token)) = command_token(&args) {
        let name = match finder::find(&token) {
            Lookup::Found(name) => name,
            Lookup::NotDefined { alternatives, .. }
                if alternatives.len() == 1 && is_interactive(&args) =>
            {
                if !confirm_alternative(&token, alternatives[0]) {
                    return 1;
                }
                alternatives[0]
            }
            Lookup::NotDefined { message, .. } | Lookup::Error(message) => {
                eprint(&render_error_block(None, &message, None));
                return 1;
            }
        };
        if !cli::PORTED_COMMANDS.contains(&name) {
            eprint(&format!(
                "lpv-rs: the \"{name}\" command of lean-package-validator is not ported to lpv-rs (yet). Supported: validate, create, update.\n"
            ));
            return 1;
        }
        args[idx] = name.to_string();
        command_name = Some(name);
    }

    let cli = match Cli::try_parse_from(&args) {
        Ok(cli) => cli,
        Err(err) => {
            return match err.kind() {
                ErrorKind::DisplayHelp
                | ErrorKind::DisplayVersion
                | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                    let _ = err.print();
                    0
                }
                _ => {
                    let message = clap_error_message(&err, command_name);
                    let synopsis = command_name.and_then(cli::synopsis_for);
                    eprint(&render_error_block(None, &message, synopsis));
                    1
                }
            };
        }
    };

    let Some(command) = cli.command else {
        let mut cmd = <Cli as clap::CommandFactory>::command();
        let _ = cmd.print_help();
        return 0;
    };

    let verbosity = if cli.quiet || cli.silent {
        Verbosity::Quiet
    } else {
        match cli.verbose {
            0 => Verbosity::Normal,
            1 => Verbosity::Verbose,
            2 => Verbosity::VeryVerbose,
            _ => Verbosity::Debug,
        }
    };
    let ansi = if cli.ansi {
        Some(true)
    } else if cli.no_ansi {
        Some(false)
    } else {
        None
    };

    let cwd = match std::env::current_dir() {
        Ok(c) => c,
        Err(e) => {
            eprint(&format!(
                "lpv-rs: cannot determine the working directory: {e}\n"
            ));
            return 1;
        }
    };

    let format = match &command {
        Command::Validate(a) => a.glob.format,
        Command::Create(a) => a.glob.format,
        Command::Update(a) => a.glob.format,
    };
    let agentic = match format {
        Format::Auto => agent::is_agentic_run(),
        Format::Text | Format::Json => false,
    };
    let out_verbosity = if format == Format::Json {
        Verbosity::Quiet
    } else {
        verbosity
    };
    let mut ctx = Ctx {
        out: Output::new(out_verbosity, ansi),
        cwd,
        agentic,
        format,
    };

    let (name, result): (&str, Result<(i32, Option<String>), LpvError>) = match &command {
        Command::Validate(a) => (
            "validate",
            commands::validate(a, &mut ctx)
                .map(|(c, r)| (c, r.and_then(|r| serde_json::to_string_pretty(&r).ok()))),
        ),
        Command::Create(a) => (
            "create",
            commands::create(a, &mut ctx)
                .map(|(c, r)| (c, r.and_then(|r| serde_json::to_string_pretty(&r).ok()))),
        ),
        Command::Update(a) => (
            "update",
            commands::update(a, &mut ctx)
                .map(|(c, r)| (c, r.and_then(|r| serde_json::to_string_pretty(&r).ok()))),
        ),
    };

    ctx.out.flush();
    match result {
        Ok((code, json)) => {
            if let Some(json) = json {
                println!("{json}");
            }
            code
        }
        Err(err) => {
            if format == Format::Json {
                let payload = serde_json::json!({
                    "tool": "lpv-rs",
                    "version": env!("CARGO_PKG_VERSION"),
                    "command": name,
                    "valid": false,
                    "exit_code": 1,
                    "error": err.to_string(),
                });
                println!("{payload:#}");
                return 1;
            }
            match &err {
                LpvError::Uncaught {
                    file,
                    line,
                    message,
                } => {
                    let title = format!("In {file} line {line}:");
                    eprint(&render_error_block(
                        Some(&title),
                        message,
                        cli::synopsis_for(name),
                    ));
                }
                LpvError::Unsupported(message) => {
                    eprint(&format!("lpv-rs: {message}\n"));
                }
                other => {
                    eprint(&render_error_block(
                        None,
                        &other.to_string(),
                        cli::synopsis_for(name),
                    ));
                }
            }
            1
        }
    }
}

fn main() -> ExitCode {
    let code = run();
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}
