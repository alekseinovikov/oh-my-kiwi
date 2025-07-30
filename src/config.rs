use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpConfig {
    host: String,
    port: u16,
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 6669,
        }
    }
}

impl TcpConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    pub fn host_str(&self) -> &str {
        &self.host
    }

    pub fn port_u16(&self) -> u16 {
        self.port
    }

    pub fn socket_addr(&self) -> std::io::Result<SocketAddr> {
        let addr = format!("{}:{}", self.host, self.port);
        addr.to_socket_addrs()?.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "cannot resolve address")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_default() {
        let config = TcpConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 6669);
    }

    #[test]
    fn test_new() {
        let config = TcpConfig::new();
        assert_eq!(config, TcpConfig::default());
    }

    #[test]
    fn test_port_builder() {
        let config = TcpConfig::new().port(8080);
        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "127.0.0.1");
    }

    #[test]
    fn test_host_builder() {
        let config = TcpConfig::new().host("localhost");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 6669);
    }

    #[test]
    fn test_chained_builders() {
        let config = TcpConfig::new().host("example.com").port(443);
        assert_eq!(config.host, "example.com");
        assert_eq!(config.port, 443);
    }

    #[test]
    fn test_set_port() {
        let mut config = TcpConfig::new();
        config.set_port(9000);
        assert_eq!(config.port, 9000);
    }

    #[test]
    fn test_set_host() {
        let mut config = TcpConfig::new();
        config.set_host("test.domain");
        assert_eq!(config.host, "test.domain");
    }

    #[test]
    fn test_host_str() {
        let config = TcpConfig::new().host("test.host");
        assert_eq!(config.host_str(), "test.host");
    }

    #[test]
    fn test_port_u16() {
        let config = TcpConfig::new().port(8888);
        assert_eq!(config.port_u16(), 8888);
    }

    #[test]
    fn test_socket_addr_localhost() {
        let config = TcpConfig::new();
        let addr = config.socket_addr().unwrap();
        assert_eq!(addr.ip(), IpAddr::from([127, 0, 0, 1]));
        assert_eq!(addr.port(), 6669);
    }

    #[test]
    fn test_socket_addr_custom() {
        let config = TcpConfig::new().host("127.0.0.1").port(8080);
        let addr = config.socket_addr().unwrap();
        assert_eq!(addr.ip(), IpAddr::from([127, 0, 0, 1]));
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_socket_addr_invalid() {
        let config = TcpConfig::new().host("invalid.domain.that.does.not.exist");
        let result = config.socket_addr();
        assert!(result.is_err());
    }
}
