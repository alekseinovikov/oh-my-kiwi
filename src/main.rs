use transport::tcp::TcpServer;
use core::config::TcpConfig;

mod core;
mod parser;
mod processor;
mod transport;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config = TcpConfig::default();
    let tcp = TcpServer::new(config);
    tcp.run().await
}
