use string_permutation::*;

fn main() {
    let word = "thought";
    let word1 = "thougth";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert!(is_permutation("abcde", "edbca"));
        assert!(!is_permutation("avcde", "edbca"));
        assert!(!is_permutation("cde", "edbca"));
        assert!(is_permutation("code", "deco"));
        assert!(!is_permutation("code", "deeco"));
        assert!(!is_permutation("codde", "deeco"));
    }

    #[test]
    fn test_repeating_characters() {
        assert!(is_permutation("aab", "baa"));
    }

    #[test]
    fn test_one_char() {
        assert!(!is_permutation("a", "b"));
        assert!(is_permutation("a", "a"));
    }

    #[test]
    fn test_empty() {
        assert!(is_permutation("", ""));
    }

    #[test]
    fn test_special_characters() {
        assert!(is_permutation("!#%@", "@%#!"));
    }

    #[test]
    fn test_unicode() {
        assert!(is_permutation("hello♥", "♥oelhl"));
        assert!(!is_permutation("♥", "♥♥"));
    }
}
