use crate::core::error::KiwiError;
use crate::core::response::Response;
use crate::core::{BytesWriter, ResponseWriter};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub(crate) struct KiwiResponseWriter<Writer: BytesWriter> {
    writer: Writer,
}

impl<Writer: BytesWriter> ResponseWriter for KiwiResponseWriter<Writer> {
    async fn write(&mut self, response: Response) -> Result<(), KiwiError> {
        self.write(response).await
    }
}

impl<Writer: BytesWriter> KiwiResponseWriter<Writer> {
    pub(crate) fn new(writer: Writer) -> Self {
        Self { writer }
    }

    pub(crate) async fn write(&mut self, response: Response) -> Result<(), KiwiError> {
        let types = response.to_types();
        let bytes = types.to_bytes();
        self.writer.write_all(&bytes).await?;

        Ok(())
    }
}

pub(crate) struct TcpBytesWriter {
    writer: Arc<Mutex<TcpStream>>,
}

impl TcpBytesWriter {
    pub(crate) fn new(writer: Arc<Mutex<TcpStream>>) -> Self {
        Self { writer }
    }
}

impl BytesWriter for TcpBytesWriter {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), KiwiError> {
        let mut writer = self.writer.lock().await;
        writer.write_all(&bytes).await?;
        Ok(writer.flush().await?)
    }
}
