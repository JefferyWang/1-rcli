use super::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
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
