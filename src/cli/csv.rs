use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use anyhow::Result;
use clap::Parser;

use crate::{process_csv, CmdExector};

use super::verify_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file, help = "输入 CSV 文件")]
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

impl CmdExector for CsvOpts {
    async fn execute(self) -> Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, output, self.format)
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
