use crate::core::error::KiwiError;
use crate::core::BytesWriter;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub(crate) struct TcpBytesWriter {
    writer: Arc<Mutex<TcpStream>>,
}

impl TcpBytesWriter {
    pub(crate) fn new(writer: Arc<Mutex<TcpStream>>) -> Self {
        Self { writer }
    }
}

#[async_trait]
impl BytesWriter for TcpBytesWriter {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), KiwiError> {
        let mut writer = self.writer.lock().await;
        writer.write_all(&bytes).await?;
        Ok(writer.flush().await?)
    }
}
