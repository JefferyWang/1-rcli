use std::path::Path;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "输入 CSV 文件")]
    pub input: String,

    #[arg(short, long, default_value = "output.json", help = "输出文件")]
    pub output: String,

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
