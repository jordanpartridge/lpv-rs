use thiserror::Error;

#[derive(Debug, Error)]
pub enum LpvError {
    /// An exception the PHP original does not catch; Symfony renders it as
    /// a framed block on stderr followed by the command synopsis.
    #[error("{message}")]
    Uncaught {
        file: &'static str,
        line: u32,
        message: String,
    },
    #[error("No license file present in archive.")]
    NoLicenseFilePresent,
    #[error("{0}")]
    Io(String),
    /// A feature of lean-package-validator that lpv-rs does not port (yet).
    #[error("{0}")]
    Unsupported(String),
}
