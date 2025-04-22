use doubtful::*;

fn main() {
    let mut s = "Hello".to_owned();

    println!("Before changing the string: {}", s);

    doubtful(&mut s);

    println!("After changing the string: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let mut s = "hello".to_owned();
        let s_copy = s.clone();

        doubtful(&mut s);

        assert_eq!(s, s_copy + "?");
    }
}
