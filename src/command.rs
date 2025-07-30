use crate::types::Types;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::info;

pub(crate) enum Command {
    None,
    Ping,
    Set { key: Types, value: Types },
}

pub(crate) struct CommandProcessor {
    stream: Arc<Mutex<TcpStream>>,
    reader: BufferedReader,
}

impl CommandProcessor {
    pub(crate) fn new(stream: TcpStream) -> Self {
        let stream = Arc::new(Mutex::new(stream));
        let reader = BufferedReader::new(stream.clone());
        Self { stream, reader }
    }

    pub(crate) async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let token = self.reader.get_next_token().await;
            let s: &str = std::str::from_utf8(&token).unwrap();
            info!("{:?}", s);
        }
    }
}

struct BufferedReader {
    reader: Arc<Mutex<TcpStream>>,
    buffer: Vec<u8>,
    pointer: usize,
}

impl BufferedReader {
    fn new(reader: Arc<Mutex<TcpStream>>) -> Self {
        Self {
            reader,
            buffer: Vec::with_capacity(1024 * 1024),
            pointer: 0,
        }
    }

    async fn get_next_token(&mut self) -> Vec<u8> {
        loop {
            if self.buffer.is_empty() || self.pointer >= self.buffer.len() - 1 {
                let mut buf = [0u8; 256];
                let mut n: std::io::Result<usize> = Ok(0);
                {
                    // lock for a short time
                    let mut reader = self.reader.lock().await;
                    n = reader.read(&mut buf).await;
                }

                match n {
                    Ok(read_size) => {
                        self.buffer.extend_from_slice(&buf[..read_size]);
                    }
                    Err(err) => panic!("${err:?}"),
                }
            }

            if let Some(token) = self.read_token_from_buffer() {
                return token;
            }

            if self.buffer.len() >= 1024 * 1024 {
                panic!("The command is too long.");
            }
        }
    }

    fn read_token_from_buffer(&mut self) -> Option<Vec<u8>> {
        let mut second_pointer = self.pointer + 1;
        while second_pointer < self.buffer.len() {
            if self.buffer[self.pointer] == b'\r' && self.buffer[second_pointer] == b'\n' {
                let token = self.buffer[0..self.pointer].to_vec();
                self.pointer = 0;
                self.buffer = self.buffer.split_off(second_pointer + 1);
                return Some(token);
            }

            self.pointer += 1;
            second_pointer += 1;
        }

        None
    }
}
