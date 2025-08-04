use crate::core::error::KiwiError;
use crate::core::BytesWriter;
use async_trait::async_trait;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;

pub(crate) struct TcpBytesWriter {
    writer: WriteHalf<TcpStream>,
}

impl TcpBytesWriter {
    pub(crate) fn new(writer: WriteHalf<TcpStream>) -> Self {
        Self { writer }
    }
}

#[async_trait]
impl BytesWriter for TcpBytesWriter {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), KiwiError> {
        self.writer.write_all(&bytes).await?;
        Ok(self.writer.flush().await?)
    }
}
