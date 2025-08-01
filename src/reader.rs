// Поместите это в ваш файл с ридером
use anyhow::anyhow;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub(crate) trait BytesReader {
    async fn read_line(&mut self) -> anyhow::Result<Vec<u8>>;
    async fn read_bytes(&mut self, n: usize) -> anyhow::Result<Vec<u8>>;
}

pub(crate) struct BufferedReader {
    reader: Arc<Mutex<TcpStream>>,
    buffer: Vec<u8>,
}

impl BufferedReader {
    pub(crate) fn new(reader: Arc<Mutex<TcpStream>>) -> Self {
        Self {
            reader,
            buffer: Vec::with_capacity(1024 * 1024), // 1MB buffer
        }
    }

    async fn ensure_buffer(&mut self, size: usize) -> anyhow::Result<()> {
        while self.buffer.len() < size {
            let mut buf = [0u8; 1024];
            let mut reader_lock = self.reader.lock().await;
            let n = reader_lock.read(&mut buf).await?;
            if n == 0 {
                return Err(anyhow!("Connection closed by peer"));
            }
            self.buffer.extend_from_slice(&buf[..n]);
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl BytesReader for BufferedReader {
    async fn read_line(&mut self) -> anyhow::Result<Vec<u8>> {
        loop {
            if let Some(pos) = self.buffer.windows(2).position(|w| w == b"\r\n") {
                let line = self.buffer[..pos].to_vec();
                self.buffer.drain(..pos + 2); // Удаляем строку и \r\n
                return Ok(line);
            }

            self.ensure_buffer(self.buffer.len() + 256).await?;
        }
    }

    async fn read_bytes(&mut self, n: usize) -> anyhow::Result<Vec<u8>> {
        self.ensure_buffer(n).await?;
        let bytes = self.buffer[..n].to_vec();
        self.buffer.drain(..n);
        Ok(bytes)
    }
}
