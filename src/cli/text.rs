use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use tokio::fs;

use crate::{process_text_generate, process_text_sign, process_text_verify, CmdExector};

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),

    #[command(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOpts),

    #[command(name = "generate", about = "Generate a key")]
    Generate(TextKeyGenerateOpts),
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_text_verify(&self.input, &self.key, &self.sig, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExector for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0]).await?;
                fs::write(name.join("ed25519.pk"), &key[1]).await?;
            }
        }
        Ok(())
    }
}

impl CmdExector for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "Input file")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file, help = "Key file")]
    pub key: String,

    #[arg(short, long, value_parser=parse_format, default_value = "blake3", help = "Hash algorithm")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "Input file")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file, help = "Key file")]
    pub key: String,

    #[arg(short, long, help = "Signature")]
    pub sig: String,

    #[arg(short, long, value_parser=parse_format, default_value = "blake3", help = "Hash algorithm")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser=parse_format, default_value = "blake3", help = "Hash algorithm")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser=verify_path, help = "Output file")]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => anyhow::bail!("Invalid format: {}", s),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
