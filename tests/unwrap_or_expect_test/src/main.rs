use unwrap_or_expect::*;

fn main() {
    println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
    println!("{}", fetch_data(Err("server.com"), Security::Warning));
    println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

    // Panics with no custom message
    // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

    // Panics with the message "ERROR: program stops"
    // fetch_data(Err("server.com"), Security::Message);

    // Panics with the message "malicious_server.com"
    // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_expect() {
        fetch_data(Err(""), Security::Message);
    }
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\"")]
    fn test_unwrap() {
        fetch_data(Err("ERROR CRITICAL"), Security::Unknown);
    }
    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn test_unwrap_err() {
        fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
    }
    #[test]
    fn test_unwrap_or() {
        assert_eq!(
            fetch_data(Err(""), Security::Warning),
            "WARNING: check the server".to_string()
        );
    }
    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(
            fetch_data(Err("another_server.com"), Security::NotFound),
            "Not found: another_server.com".to_string()
        );
    }
    #[test]
    fn test_ok() {
        assert_eq!(
            fetch_data(Ok("server.com"), Security::Message),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com"), Security::Warning),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com"), Security::NotFound),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com"), Security::Unknown),
            "server.com"
        );
    }
}
