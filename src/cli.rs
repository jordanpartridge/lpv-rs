//! Command line definition. Flags and value semantics follow the Symfony
//! Console definitions of lean-package-validator v6.3.2.

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum Format {
    /// Human output, or lpv's agent JSON when a coding agent is detected.
    #[default]
    Auto,
    /// Always human output.
    Text,
    /// Rich machine-readable JSON report (lpv-rs extension).
    Json,
}

#[derive(Debug, Parser)]
#[command(
    name = "lpv-rs",
    version,
    about = "Lean package validator (Rust port of raphaelstolt/lean-package-validator 6.3.2)",
    disable_help_subcommand = true,
    args_override_self = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Increase the verbosity of messages: 1 for normal output, 2 for more verbose output and 3 for debug
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Do not output any message
    #[arg(short = 'q', long = "quiet", global = true)]
    pub quiet: bool,

    /// Do not output any message (alias of --quiet)
    #[arg(long = "silent", global = true)]
    pub silent: bool,

    /// Do not ask any interactive question (accepted for compatibility)
    #[arg(short = 'n', long = "no-interaction", global = true)]
    pub no_interaction: bool,

    /// Force ANSI output
    #[arg(long = "ansi", global = true)]
    pub ansi: bool,

    /// Disable ANSI output
    #[arg(long = "no-ansi", global = true)]
    pub no_ansi: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Validate the .gitattributes file of a given project/micro-package repository
    Validate(ValidateArgs),
    /// Create a new .gitattributes file for a project/micro-package repository
    Create(CreateArgs),
    /// Update an existing .gitattributes file for a project/micro-package repository
    Update(UpdateArgs),
}

#[derive(Debug, Args, Clone)]
pub struct GlobArgs {
    /// Use this glob pattern e.g. {.*,*.md} to match artefacts which should be export-ignored
    #[arg(
        long = "glob-pattern",
        value_name = "GLOB-PATTERN",
        allow_hyphen_values = false
    )]
    pub glob_pattern: Option<String>,

    /// Use this file with glob patterns to match artifacts which should be export-ignored
    #[arg(long = "glob-pattern-file", value_name = "GLOB-PATTERN-FILE", num_args = 0..=1, default_missing_value = "")]
    pub glob_pattern_file: Option<String>,

    /// Use the glob pattern of the given language preset
    #[arg(long = "preset", value_name = "PRESET", num_args = 0..=1, default_missing_value = "")]
    pub preset: Option<String>,

    /// Do not export-ignore the license file
    #[arg(long = "keep-license")]
    pub keep_license: bool,

    /// Do not export-ignore the README file
    #[arg(long = "keep-readme")]
    pub keep_readme: bool,

    /// Sort from directories to files
    #[arg(short = 's', long = "sort-from-directories-to-files")]
    pub sort_from_directories_to_files: bool,

    /// Enforce a strict order comparison of export-ignores in the .gitattributes file
    #[arg(long = "enforce-strict-order")]
    pub enforce_strict_order: bool,

    /// Output format: auto (lpv-compatible), text, or json (lpv-rs extension)
    #[arg(long = "format", value_enum, default_value_t = Format::Auto)]
    pub format: Format,
}

#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// The directory of a project/micro-package repository
    pub directory: Option<String>,

    /// Read .gitattributes content from standard input
    #[arg(long = "stdin-input")]
    pub stdin_input: bool,

    /// Enforce a strict alignment of export-ignores in the .gitattributes file
    #[arg(long = "enforce-alignment")]
    pub enforce_alignment: bool,

    /// Validate Git archive against current HEAD
    #[arg(long = "validate-git-archive")]
    pub validate_git_archive: bool,

    /// Accepted as a value-less flag, exactly like lpv's validate command (where it is
    /// always rejected as an invalid glob pattern)
    #[arg(long = "keep-glob-pattern")]
    pub keep_glob_pattern: bool,

    /// Show difference between expected and actual .gitattributes content
    #[arg(long = "diff")]
    pub diff: bool,

    /// Filter stale export-ignores referencing non existent artifacts
    #[arg(long = "report-stale-export-ignores")]
    pub report_stale_export_ignores: bool,

    /// Display the distribution impact report (not ported to lpv-rs)
    #[arg(long = "report-impact")]
    pub report_impact: bool,

    #[command(flatten)]
    pub glob: GlobArgs,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The package directory to create the .gitattributes file in
    pub directory: Option<String>,

    /// Do not export-ignore matching glob pattern e.g. {LICENSE.*,README.*,docs*}
    #[arg(long = "keep-glob-pattern", value_name = "KEEP-GLOB-PATTERN")]
    pub keep_glob_pattern: Option<String>,

    /// Align export-ignores on create or overwrite
    #[arg(short = 'a', long = "align-export-ignores")]
    pub align_export_ignores: bool,

    /// Do not write any files. Output the expected .gitattributes content
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Force the creation of the .gitattributes file even if one already exists
    #[arg(long = "force")]
    pub force: bool,

    /// Generate the .gitattributes file with the given flavour (classic; negated is not ported)
    #[arg(short = 'f', long = "flavour", value_name = "FLAVOUR", num_args = 0..=1, default_missing_value = "")]
    pub flavour: Option<String>,

    #[command(flatten)]
    pub glob: GlobArgs,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// The package directory whose .gitattributes file should be updated
    pub directory: Option<String>,

    /// Only reformat the export-ignores directives (accepted; a no-op in lpv 6.3.2 too)
    #[arg(long = "reformat-export-ignores")]
    pub reformat_export_ignores: bool,

    /// Group non export-ignore directives in a separate section
    #[arg(long = "group")]
    pub group: bool,

    /// Do not export-ignore matching glob pattern e.g. {LICENSE.*,README.*,docs*}
    #[arg(long = "keep-glob-pattern", value_name = "KEEP-GLOB-PATTERN")]
    pub keep_glob_pattern: Option<String>,

    /// Align export-ignores on create or overwrite
    #[arg(short = 'a', long = "align-export-ignores")]
    pub align_export_ignores: bool,

    /// Do not write any files. Output the expected .gitattributes content
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    #[command(flatten)]
    pub glob: GlobArgs,
}

pub const VALIDATE_SYNOPSIS: &str = "validate [--stdin-input] [--enforce-strict-order] [--enforce-alignment] [--validate-git-archive] [--glob-pattern GLOB-PATTERN] [--glob-pattern-file [GLOB-PATTERN-FILE]] [--preset [PRESET]] [--keep-license] [--keep-readme] [--keep-glob-pattern] [-s|--sort-from-directories-to-files] [--diff] [--report-stale-export-ignores] [--report-impact] [--] [<directory>]";
pub const CREATE_SYNOPSIS: &str = "create [--glob-pattern GLOB-PATTERN] [--glob-pattern-file [GLOB-PATTERN-FILE]] [--preset [PRESET]] [--keep-license] [--keep-readme] [--keep-glob-pattern KEEP-GLOB-PATTERN] [-a|--align-export-ignores] [-s|--sort-from-directories-to-files] [--enforce-strict-order] [--dry-run] [--force] [-f|--flavour [FLAVOUR]] [--] [<directory>]";
pub const UPDATE_SYNOPSIS: &str = "update [--reformat-export-ignores] [--group] [--glob-pattern GLOB-PATTERN] [--glob-pattern-file [GLOB-PATTERN-FILE]] [--preset [PRESET]] [--keep-license] [--keep-readme] [--keep-glob-pattern KEEP-GLOB-PATTERN] [-a|--align-export-ignores] [-s|--sort-from-directories-to-files] [--enforce-strict-order] [--dry-run] [--] [<directory>]";

/// A command registered by lean-package-validator 6.3.2.
pub struct LpvCommand {
    pub name: &'static str,
    pub hidden: bool,
    pub description: &'static str,
}

const fn lpv_command(name: &'static str, hidden: bool, description: &'static str) -> LpvCommand {
    LpvCommand {
        name,
        hidden,
        description,
    }
}

/// Every command lean-package-validator 6.3.2 registers, in registration
/// order (Symfony's defaults first, then `bin/lean-package-validator`'s
/// `addCommands()` list). The order drives the ambiguity suggestions.
pub const LPV_COMMANDS: &[LpvCommand] = &[
    lpv_command("help", false, "Display help for a command"),
    lpv_command("list", false, "List commands"),
    lpv_command(
        "_complete",
        true,
        "Internal command to provide shell completion suggestions",
    ),
    lpv_command("completion", false, "Dump the shell completion script"),
    lpv_command(
        "init",
        false,
        "Create a default .lpv file in a given project/micro-package repository",
    ),
    lpv_command("refresh", false, "Refresh a present .lpv file"),
    lpv_command(
        "validate",
        false,
        "Validate the .gitattributes file of a given project/micro-package repository",
    ),
    lpv_command(
        "validate-remote",
        false,
        "Validate the .gitattributes file of a remote GitHub repository and create or update it in a local clone",
    ),
    lpv_command(
        "create",
        false,
        "Create a new .gitattributes file for a project/micro-package repository",
    ),
    lpv_command(
        "update",
        false,
        "Update an existing .gitattributes file for a project/micro-package repository",
    ),
    lpv_command("reformat", false, "Reformat a present .gitattributes file"),
    lpv_command(
        "tree",
        false,
        "Display the source structure of a given project/micro-package repository or it's dist package",
    ),
    lpv_command("list-skills", false, "List included AI skills"),
];

pub const PORTED_COMMANDS: &[&str] = &["create", "update", "validate"];

pub fn synopsis_for(command: &str) -> Option<&'static str> {
    match command {
        "validate" => Some(VALIDATE_SYNOPSIS),
        "create" => Some(CREATE_SYNOPSIS),
        "update" => Some(UPDATE_SYNOPSIS),
        _ => None,
    }
}
