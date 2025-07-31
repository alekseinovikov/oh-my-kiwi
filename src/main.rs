use crate::config::TcpConfig;
use crate::tcp::TcpServer;

mod config;
mod parser;
mod processor;
mod reader;
mod server;
mod tcp;
mod types;
mod writer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = TcpConfig::default();
    let tcp = TcpServer::new(config);
    tcp.run().await
}
