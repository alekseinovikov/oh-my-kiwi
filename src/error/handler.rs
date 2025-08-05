use crate::core::response::Response;
use crate::core::{ErrorHandler, ResponseWriter};
use crate::error::{KiwiError, ParseError};
use async_trait::async_trait;
use tracing::info;

pub(crate) struct KiwiErrorHandler;

impl KiwiErrorHandler {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[async_trait]
impl<RW> ErrorHandler<RW> for KiwiErrorHandler
where
    RW: ResponseWriter + Send + Sync + 'static,
{
    async fn handle_error(&self, response_writer: &mut RW, error: KiwiError) -> Option<KiwiError> {
        let response = self.map_err_to_response_if_possible(error);

        if let Ok(Some(response)) = response {
            let error = Self::write_error_response(response_writer, response).await;
            if let Err(err) = error {
                return Some(err);
            }
        } else if let Err(err) = response {
            return Some(err);
        };

        None
    }
}

impl KiwiErrorHandler {
    fn map_err_to_response_if_possible(
        &self,
        err: KiwiError,
    ) -> Result<Option<Response>, KiwiError> {
        match err {
            KiwiError::ParseError(err) => {
                if let ParseError::ConnectionClosed = err {
                    info!("Connection closed");
                    Err(KiwiError::ConnectionClosed)
                } else {
                    Ok(Some(Response::Error(err.to_string())))
                }
            }
            KiwiError::CommandError(err) => Ok(Some(Response::Error(err.to_string()))),
            KiwiError::ConnectionError(err) => Ok(Some(Response::Error(err.to_string()))),
            KiwiError::ConnectionClosed => Err(KiwiError::ConnectionClosed),
        }
    }

    async fn write_error_response<RW>(
        response_writer: &mut RW,
        response: Response,
    ) -> Result<(), KiwiError>
    where
        RW: ResponseWriter,
    {
        response_writer.write(response).await
    }
}
