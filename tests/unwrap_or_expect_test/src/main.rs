use unwrap_or_expect::*;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_expect() {
        fetch_data(Err(String::new()), Security::High);
    }
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\"")]
    fn test_unwrap() {
        fetch_data(Err("ERROR CRITICAL".to_string()), Security::Unknown);
    }
    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn test_unwrap_err() {
        fetch_data(
            Ok("malicious_server.com".to_string()),
            Security::BlockServer,
        );
    }
    #[test]
    fn test_unwrap_or() {
        assert_eq!(
            fetch_data(Err(String::new()), Security::Medium),
            "WARNING: check the server".to_string()
        );
    }
    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(
            fetch_data(Err("another_server.com".to_string()), Security::Low),
            "Not found: another_server.com".to_string()
        );
    }
    #[test]
    fn test_ok() {
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Low),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Medium),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::High),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Unknown),
            "server.com"
        );
    }
}
