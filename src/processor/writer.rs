use crate::core::error::KiwiError;
use crate::core::response::Response;
use crate::core::{BytesWriter, ResponseWriter};
use async_trait::async_trait;

pub(crate) struct KiwiResponseWriter<Writer: BytesWriter> {
    writer: Writer,
}

#[async_trait]
impl<Writer: BytesWriter + Send> ResponseWriter for KiwiResponseWriter<Writer> {
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
