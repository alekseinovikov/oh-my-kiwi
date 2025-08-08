use oh_my_kiwi_domain::error::ParseError;
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;
use oh_my_kiwi_domain::BytesReader;

pub struct TcpBufferedReader {
    reader: ReadHalf<TcpStream>,
    buffer: Vec<u8>,
}

impl TcpBufferedReader {
    pub fn new(reader: ReadHalf<TcpStream>) -> Self {
        Self {
            reader,
            buffer: Vec::with_capacity(1024 * 1024), // 1MB buffer
        }
    }

    async fn ensure_buffer(&mut self) -> Result<(), ParseError> {
        while self.buffer.is_empty() {
            let mut buf = [0u8; 1024];
            let n = self.reader.read(&mut buf).await;
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

#[async_trait]
impl BytesReader for TcpBufferedReader {
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
