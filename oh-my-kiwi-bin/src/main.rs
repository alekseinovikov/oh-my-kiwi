use oh_my_kiwi_domain::error::KiwiErrorHandler;
use oh_my_kiwi_engine::command_processor::KiwiCommandProcessor;
use oh_my_kiwi_engine::in_memory::InMemoryEngine;
use oh_my_kiwi_engine::response_writer::KiwiResponseWriter;
use oh_my_kiwi_parser::KiwiCommandParser;
use oh_my_kiwi_tcp::start_tcp_server;
use std::sync::Arc;

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
