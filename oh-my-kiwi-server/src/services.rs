use async_trait::async_trait;
use oh_my_kiwi_domain::command::KiwiCommand;
use oh_my_kiwi_domain::error::KiwiError;

#[async_trait]
pub trait CommandParser {
    async fn parse_next_command(&mut self) -> Result<KiwiCommand, KiwiError>;
}

