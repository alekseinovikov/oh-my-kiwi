use crate::types::Types;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::error;

pub(crate) enum Command {
    None,
    Ping,
    Set { key: Types, value: Types },
}

pub(crate) struct CommandProcessor {
    reader: BufferedReader,
}

impl CommandProcessor {
    pub(crate) fn new(stream: TcpStream) -> Self {
        Self {
            reader: BufferedReader::new(stream),
        }
    }
}

pub(crate) async fn parse_user_command(mut reader: TcpStream) -> anyhow::Result<Command> {
    let reader = BufferedReader::new(reader);

    Ok(Command::None)
}

struct BufferedReader {
    reader: TcpStream,
    buffer: Vec<u8>,
    pointer: usize,
}

impl BufferedReader {
    fn new(reader: TcpStream) -> Self {
        Self {
            reader,
            buffer: Vec::with_capacity(1024 * 1024),
            pointer: 0,
        }
    }

    fn read_next(&mut self, n: usize) -> Option<Vec<u8>> {
        self.try_to_read_next();
        if self.get_buffer_tail_size() < n {
            return None;
        }

        let result = self.buffer.as_slice()[self.pointer..self.pointer + n].to_vec();
        self.pointer += n;
        Some(result)
    }

    async fn try_to_read_next(&mut self) {
        self.read_to_buffer_available(false).await;
    }

    async fn reset_and_await_data(&mut self) {
        self.try_to_read_next().await;
        self.pointer = 0;
        self.buffer.clear();
        self.read_to_buffer_available(true).await;
    }

    async fn read_to_buffer_available(&mut self, blocking: bool) {
        let mut buf = [0u8; 256];
        let n = if blocking {
            self.reader.read(&mut buf).await
        } else {
            self.reader.try_read(&mut buf)
        };

        match n {
            Ok(size) => {
                if size > 0 {
                    self.buffer.extend_from_slice(&buf[..size]);
                }
            }
            Err(err) => {
                error!("Failed to read from socket: {}", err);
                self.buffer.clear();
            }
        }
    }

    fn get_buffer_tail_size(&self) -> usize {
        self.buffer.len() - self.pointer
    }
}
