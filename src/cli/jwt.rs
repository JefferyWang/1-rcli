use std::time::SystemTime;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use humantime::parse_duration;
use serde::{Deserialize, Serialize};

use crate::{
    process::{process_jwt_sign, process_jwt_verify},
    CmdExector,
};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Generate a JWT token")]
    Sign(JwtSignOpts),

    #[command(name = "verify", about = "Verify a JWT")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
    pub exp: u64,
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, default_value = "your-256-bit-secret", help = "secret data")]
    pub secret: String,

    #[arg(long, help = "sub data")]
    pub sub: String,

    #[clap(long, help = "aud data")]
    pub aud: String,

    #[clap(long, value_parser = parse_exp, default_value= "1d", help = "exp data")]
    pub exp: u64,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(
        short,
        long,
        default_value = "your-256-bit-secret",
        help = "secret data"
    )]
    pub secret: String,

    #[arg(short, long, help = "JWT Token")]
    pub token: String,
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_jwt_sign(
            &self.secret,
            &Claims {
                sub: self.sub,
                aud: self.aud,
                exp: self.exp,
            },
        )?;
        println!("{}", token);
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_jwt_verify(&self.secret, &self.token)?;
        println!("{:?}", ret);
        Ok(())
    }
}

pub fn parse_exp(s: &str) -> Result<u64, anyhow::Error> {
    let duration = parse_duration(s)?;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    Ok(now + duration.as_secs())
}
