use bigger::*;
use std::collections::HashMap;

fn main() {
    let hash = HashMap::from_iter([
        ("Daniel", 122),
        ("Ashley", 333),
        ("Katie", 334),
        ("Robert", 14),
    ]);

    println!(
        "The biggest of the elements in the HashMap is {}",
        bigger(hash)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        let f = HashMap::from_iter([
            ("Daniel", 122),
            ("Ashley", 333),
            ("Katie", 334),
            ("Robert", 14),
        ]);

        assert_eq!(334, bigger(f));
    }

    #[test]
    fn test_long() {
        let f = HashMap::from_iter([
            ("Daniel", 41758712),
            ("Ashley", 54551444),
            ("Katie", 575556334),
            ("Robert", 574148),
        ]);

        assert_eq!(575556334, bigger(f));
    }
}
