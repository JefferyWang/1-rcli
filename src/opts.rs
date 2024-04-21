use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    str::FromStr,
};

use clap::{Parser, Subcommand};

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "输入 CSV 文件")]
    pub input: String,

    #[arg(short, long, help = "输出文件")]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format, default_value = "json", help = "输出文件格式")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',', help = "CSV 文件的分隔符")]
    pub delimiter: char,

    #[arg(long, default_value_t = true, help = "CSV 是否包含标题行")]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => anyhow::bail!("Invalid format: {}", s),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16, help = "密码长度")]
    pub length: u8,

    #[arg(long, default_value_t = true, help = "包含大写字母")]
    pub uppercase: bool,

    #[arg(long, default_value_t = true, help = "包含小写字母")]
    pub lowercase: bool,

    #[arg(long, default_value_t = true, help = "包含数字")]
    pub number: bool,

    #[arg(long, default_value_t = true, help = "包含符号")]
    pub symbol: bool,
}
