use to_url::*;

fn main() {
    let s = "Hello, world!";
    println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        assert_eq!(to_url("this is my search"), "this%20is%20my%20search");
        assert_eq!(to_url("another search "), "another%20search%20");
    }
}
