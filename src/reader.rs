use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub(crate) enum Token {
    String(String),
    Null,
    Boolean(bool),
}

pub(crate) struct BufferedReader {
    reader: TcpStream,
    buffer: Vec<u8>,
    pointer: usize,
}

impl BufferedReader {
    pub(crate) fn new(reader: TcpStream) -> Self {
        Self {
            reader,
            buffer: Vec::with_capacity(1024 * 1024),
            pointer: 0,
        }
    }

    pub(crate) async fn get_next_token(&mut self) -> Vec<u8> {
        loop {
            if self.buffer.is_empty() || self.pointer >= self.buffer.len() - 1 {
                let mut buf = [0u8; 256];
                let n: std::io::Result<usize> = self.reader.read(&mut buf).await;
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
