use crate::config::TcpConfig;
use crate::server::RESP3Server;
use crate::tcp::TcpServer;

mod config;
mod server;
mod tcp;
mod command;
mod types;
mod response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = TcpConfig::default();
    let server = RESP3Server::new();
    let tcp = TcpServer::new(config, server);
    tcp.run().await
}
