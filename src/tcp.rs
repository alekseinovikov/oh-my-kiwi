use crate::config::TcpConfig;
use crate::server::Server;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::info;
use crate::command::parse_user_command;

pub(crate) struct TcpServer<S: Server + Send> {
    tcp_config: TcpConfig,
    server: Arc<Mutex<S>>,
}

impl<S: Server + Send + 'static> TcpServer<S> {
    pub(crate) fn new(tcp_config: TcpConfig, server: S) -> TcpServer<S> {
        Self {
            tcp_config,
            server: Arc::new(Mutex::new(server)),
        }
    }

    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let socket_addr = self.tcp_config.socket_addr()?;
        let listener = TcpListener::bind(socket_addr).await?;

        loop {
            let (mut stream, addr) = listener.accept().await?;
            info!("New connection from: ${addr}");

            let server = self.server.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_client(stream, server).await {
                    eprintln!("Error with {}: {:?}", addr, e);
                }
            });
        }
    }
}

async fn handle_client<S: Server + Send>(
    mut stream: TcpStream,
    server: Arc<Mutex<S>>,
) -> anyhow::Result<()> {
    let command = parse_user_command(stream).await?;
   
    Ok(())
}
