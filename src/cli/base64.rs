use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_decode, process_encode, CmdExector};

use super::verify_file;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Base64 decode")]
    Decode(Base64DecodeOpts),
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> Result<()> {
        let encoded = process_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        // TODO: decoded data might not be string (but for this example, we assume it is)
        let decoded = String::from_utf8(decoded)?;
        println!("{}", decoded);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser=verify_file,default_value="-", help = "输入文件")]
    pub input: String,

    #[arg(short, long, value_parser=parse_base64_format, default_value = "standard", help = "Base64 格式")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser=verify_file,default_value="-", help = "输入文件")]
    pub input: String,

    #[arg(short, long, value_parser=parse_base64_format, default_value = "standard", help = "Base64 格式")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => anyhow::bail!("Invalid format: {}", s),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
