pub(crate) trait Server {
    fn process_line(&self, line: &str) -> String;
}

#[derive(Debug, Default)]
pub(crate) struct RESP3Server;

impl RESP3Server {
    pub(crate) const fn new() -> Self { Self }
}

impl Server for RESP3Server {
    fn process_line(&self, line: &str) -> String {
        let cmd = line.trim_end_matches(|c| c == '\r' || c == '\n');
        if cmd.eq_ignore_ascii_case("PING") {
            "PONG\r\n".into()
        } else if cmd.eq_ignore_ascii_case("QUIT") {
            "BYE\r\n".into()
        } else {
            "UNKNOWN COMMAND\r\n".into()
        }
    }
}
