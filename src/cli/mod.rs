mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::Path;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use self::{csv::CsvOpts, genpass::GenPassOpts};

pub use self::base64::Base64Format;
pub use self::base64::Base64SubCommand;
pub use self::csv::OutputFormat;
pub use self::http::HttpSubCommand;
pub use self::text::TextSignFormat;
pub use self::text::TextSubCommand;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats.")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand),

    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

fn verify_path(path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err(format!("File not found or is not a directory: {}", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_file("not-exist"),
            Err("File not found: not-exist".into())
        );
    }
}
