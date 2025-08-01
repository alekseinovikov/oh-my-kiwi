use crate::error::KiwiError;
use crate::processor::Response;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub(crate) struct ResponseWriter {
    writer: Arc<Mutex<TcpStream>>,
}

impl ResponseWriter {
    pub(crate) fn new(writer: Arc<Mutex<TcpStream>>) -> Self {
        Self { writer }
    }

    pub(crate) async fn write(&mut self, response: Response) -> Result<(), KiwiError> {
        let types = response.to_types();
        let bytes = types.to_bytes();
        
        {
            let mut writer = self.writer.lock().await;
            writer.write_all(&bytes).await
        }?;

        Ok(())
    }
}
