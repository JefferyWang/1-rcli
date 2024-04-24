mod cli;
mod process;
mod utils;

use anyhow::Result;
pub use cli::{Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
pub use process::process_csv;
pub use process::process_decode;
pub use process::process_encode;
pub use process::process_genpass;
pub use process::process_http_serve;
pub use process::process_text_generate;
pub use process::process_text_sign;
pub use process::process_text_verify;
pub use utils::get_reader;

#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> Result<()>;
}
