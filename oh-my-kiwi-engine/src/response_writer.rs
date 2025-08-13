use async_trait::async_trait;
use oh_my_kiwi_domain::{BytesWriter, ResponseWriter};
use oh_my_kiwi_domain::error::KiwiError;
use oh_my_kiwi_domain::response::Response;

pub struct KiwiResponseWriter<Writer: BytesWriter> {
    writer: Writer,
}

#[async_trait]
impl<Writer: BytesWriter + Send> ResponseWriter for KiwiResponseWriter<Writer> {
    async fn write(&mut self, response: Response) -> Result<(), KiwiError> {
        self.write(response).await
    }
}

impl<Writer: BytesWriter> KiwiResponseWriter<Writer> {
    pub fn new(writer: Writer) -> Self {
        Self { writer }
    }

    pub(crate) async fn write(&mut self, response: Response) -> Result<(), KiwiError> {
        let types = response.to_types();
        let bytes = types.to_bytes();
        self.writer.write_all(&bytes).await?;

        Ok(())
    }
}
