use is_anagram::*;

fn main() {
    println!("{}", is_anagram("listen", "silent"));
    println!("{}", is_anagram("RaceCar", "racecar"));
    println!("{}", is_anagram("hello world", "world hello"));
}

#[cfg(test)]
mod tests {
    use is_anagram::*;

    #[test]
    fn basic_anagram() {
        let s1 = "listen";
        let s2 = "silent";
        assert!(is_anagram(s1, s2));
    }

    #[test]
    fn anagram_with_different_case() {
        let s1 = "RaceCar";
        let s2 = "racecar";
        assert!(is_anagram(s1, s2));
    }

    #[test]
    fn anagram_with_spaces() {
        let s1 = "hello world";
        let s2 = "world hello";
        assert!(is_anagram(s1, s2));
    }

    #[test]
    fn non_anagram_with_different_lengths() {
        let s1 = "abc";
        let s2 = "abcd";
        assert!(!is_anagram(s1, s2));
    }

    #[test]
    fn non_anagram_with_different_characters() {
        let s1 = "hello";
        let s2 = "world";
        assert!(!is_anagram(s1, s2));
    }

    #[test]
    fn anagram_with_special_characters() {
        let s1 = "!#%@";
        let s2 = "@%#!";
        assert!(is_anagram(s1, s2));
    }

    #[test]
    fn empty_strings() {
        let s1 = "";
        let s2 = "";
        assert!(is_anagram(s1, s2));
    }

    #[test]
    fn test_anagram_repeating_characters() {
        let s1 = "aab";
        let s2 = "baa";
        assert!(is_anagram(s1, s2));
    }

    #[test]
    fn non_anagram_with_one_characters() {
        let s1 = "a";
        let s2 = "b";
        assert!(!is_anagram(s1, s2));
    }

    #[test]
    fn anagram_with_unicode_characters() {
        let s1 = "🙂😀";
        let s2 = "😀🙂";
        assert!(is_anagram(s1, s2));
    }
}
