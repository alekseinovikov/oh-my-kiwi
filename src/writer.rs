use crate::processor::Response;
use tokio::net::TcpStream;

pub(crate) struct ResponseWriter {
    writer: TcpStream,
}

impl ResponseWriter {
    pub(crate) fn new(writer: TcpStream) -> Self {
        Self { writer }
    }

    pub(crate) async fn write(&mut self, response: Response) -> anyhow::Result<()> {}
}
