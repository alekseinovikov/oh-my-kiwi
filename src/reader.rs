use crate::error::{KiwiError, ParseError};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub(crate) trait BytesReader {
    async fn read_line(&mut self) -> Result<Vec<u8>, ParseError>;
    async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ParseError>;
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

    async fn ensure_buffer(&mut self) -> Result<(), ParseError> {
        while self.buffer.is_empty() {
            let mut buf = [0u8; 1024];
            let mut reader_lock = self.reader.lock().await;
            let n = reader_lock.read(&mut buf).await;
            match n {
                Ok(n) => {
                    if n == 0 {
                        return Err(ParseError::ConnectionClosed);
                    }
                    self.buffer.extend_from_slice(&buf[..n]);
                }
                Err(_) => return Err(ParseError::ConnectionClosed),
            }
        }
        Ok(())
    }
}

impl BytesReader for BufferedReader {
    async fn read_line(&mut self) -> Result<Vec<u8>, ParseError> {
        loop {
            if let Some(pos) = self.buffer.windows(2).position(|w| w == b"\r\n") {
                let line = self.buffer[..pos].to_vec();
                self.buffer.drain(..pos + 2);
                return Ok(line);
            }

            self.ensure_buffer().await?;
        }
    }

    async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ParseError> {
        let mut result = Vec::with_capacity(n);
        while result.len() < n {
            self.ensure_buffer().await?;
            let bytes = self.buffer[..n].to_vec();
            self.buffer.drain(..n);
            result.extend_from_slice(&bytes);
        }

        Ok(result)
    }
}
