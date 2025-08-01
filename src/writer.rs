use crate::processor::Response;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub(crate) struct ResponseWriter {
    writer: Arc<Mutex<TcpStream>>,
}

impl ResponseWriter {
    pub(crate) fn new(writer: Arc<Mutex<TcpStream>>) -> Self {
        Self { writer }
    }

    pub(crate) async fn write(&mut self, response: Response) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}
