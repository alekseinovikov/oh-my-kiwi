use async_trait::async_trait;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use oh_my_kiwi_domain::BytesWriter;
use oh_my_kiwi_domain::error::KiwiError;

pub struct TcpBytesWriter {
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
