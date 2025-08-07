use std::sync::Arc;
use crate::engine::in_memory::InMemoryEngine;
use crate::error::handler::KiwiErrorHandler;
use crate::parser::parser::KiwiCommandParser;
use crate::processor::processor::KiwiCommandProcessor;
use crate::processor::writer::KiwiResponseWriter;
use crate::transport::tcp::start_tcp_server;

mod core;
mod error;
mod parser;
mod processor;
mod provider;
mod server;
mod transport;
mod engine;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let engine = Arc::new(InMemoryEngine::new());

    let processor_factory = move || KiwiCommandProcessor::new(engine.clone());
    let parser_factory = move |byte_reader| KiwiCommandParser::new(byte_reader);
    let response_writer_factory = move |byte_writer| KiwiResponseWriter::new(byte_writer);
    let error_handler_factory = move || KiwiErrorHandler::new();

    start_tcp_server(
        processor_factory,
        parser_factory,
        response_writer_factory,
        error_handler_factory,
    )
    .await
}
