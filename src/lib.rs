mod cli;
mod process;
mod utils;

pub use cli::{Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
pub use process::process_csv;
pub use process::process_decode;
pub use process::process_encode;
pub use process::process_genpass;
pub use process::process_text_generate;
pub use process::process_text_sign;
pub use process::process_text_verify;
pub use utils::get_reader;
