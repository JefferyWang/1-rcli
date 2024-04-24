use anyhow::Result;
use clap::Parser;

use crate::{process_genpass, CmdExector};
use zxcvbn::zxcvbn;

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

impl CmdExector for GenPassOpts {
    async fn execute(self) -> Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", password);

        let estimate = zxcvbn(&password, &[])?;
        eprintln!("Estimated strength: {}", estimate.score());
        Ok(())
    }
}
