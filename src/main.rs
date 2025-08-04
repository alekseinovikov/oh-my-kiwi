use crate::transport::tcp::server::TcpServer;
use transport::tcp::config::TcpConfig;
use crate::processor::processor::KiwiCommandProcessor;

mod core;
mod parser;
mod processor;
mod server;
mod transport;
mod provider;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let processor_factory = move || KiwiCommandProcessor::new();

    let config = TcpConfig::default();
    let tcp = TcpServer::new(config, processor_factory);
    tcp.run().await
}
