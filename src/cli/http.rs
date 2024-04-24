use crate::{process_http_serve, CmdExector};

use super::verify_path;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".", help = "Directory to serve")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080, help = "Port to listen on")]
    pub port: u16,
}

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}
