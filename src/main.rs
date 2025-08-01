use crate::config::TcpConfig;
use crate::error::KiwiError;
use crate::tcp::TcpServer;

mod config;
mod error;
mod parser;
mod processor;
mod reader;
mod server;
mod tcp;
mod types;
mod writer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config = TcpConfig::default();
    let tcp = TcpServer::new(config);
    tcp.run().await
}
