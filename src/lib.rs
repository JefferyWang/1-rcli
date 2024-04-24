mod cli;
mod process;
mod utils;

use anyhow::Result;
pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::process_csv;
pub use process::process_decode;
pub use process::process_encode;
pub use process::process_genpass;
pub use process::process_http_serve;
pub use process::process_text_decrypt;
pub use process::process_text_encrypt;
pub use process::process_text_generate;
pub use process::process_text_sign;
pub use process::process_text_verify;
pub use utils::get_reader;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> Result<()>;
}
