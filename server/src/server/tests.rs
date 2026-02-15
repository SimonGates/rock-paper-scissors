use super::*;

#[cfg(test)]
mod server_builder_tests {
    use super::*;

    /// Test that ServerBuilder::new() creates a default instance with no port or bind address set.
    #[test]
    fn test_server_builder_new_creates_default() {
        let builder = ServerBuilder::new();

        assert!(builder.port.is_none());
        assert!(builder.bind_address.is_none());
    }

    /// Test that the port() method correctly sets the port value in the builder.
    #[test]
    fn test_server_builder_with_valid_port() {
        let builder = ServerBuilder::new().port(6767);

        assert_eq!(builder.port, Some(6767));
    }

    /// Test that build() succeeds when a valid port is provided and creates a server
    /// with the correct port and default bind address (127.0.0.1).
    #[test]
    fn test_server_builder_build_success() {
        let result = ServerBuilder::new().port(6767).build();

        assert!(result.is_ok());

        let server = result.unwrap();
        assert_eq!(server.port, 6767);
        assert_eq!(server.bind_address.ip().to_string(), "127.0.0.1");
        assert_eq!(server.bind_address.port(), 6767);
    }

    /// Test that build() returns an error when no port is set.
    #[test]
    fn test_server_builder_build_missing_port() {
        let result = ServerBuilder::new().build();

        assert!(result.is_err());

        match result.unwrap_err() {
            BuildError::MissingOrInvalidPort => {
                // Expected error type
            }
            _ => panic!("Expected BuildError::MissingOrInvalidPort"),
        }
    }

    /// Test that build() succeeds with various valid port numbers.
    #[test]
    fn test_server_builder_with_various_valid_ports() {
        let test_ports = [1, 80, 443, 8080, 3000, 65535];

        for port in test_ports {
            let result = ServerBuilder::new().port(port).build();
            assert!(result.is_ok(), "Port {} should be valid", port);

            let server = result.unwrap();
            assert_eq!(server.port, port);
        }
    }

    /// Test that the default bind address is localhost (127.0.0.1) when not explicitly set.
    #[test]
    fn test_server_builder_default_bind_address_is_localhost() {
        let server = ServerBuilder::new()
            .port(8080)
            .build()
            .expect("Server should build successfully");

        assert_eq!(server.bind_address.ip().to_string(), "127.0.0.1");
    }
}
