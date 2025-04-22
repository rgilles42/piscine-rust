use simple_hash::*;

const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

fn main() {
    let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
    let frequency_count = word_frequency_counter(&words);

    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn solution<'a>(words: &'a [&str]) -> HashMap<&'a str, usize> {
        let mut hash = HashMap::with_capacity(words.len() / 2);

        words
            .iter()
            .copied()
            .for_each(|w| *hash.entry(w).or_default() += 1);

        hash
    }

    #[test]
    fn test_basic_example() {
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();

        assert_eq!(solution(&words), word_frequency_counter(&words));
    }

    #[test]
    fn test_frequency_counter() {
        let words = "on the dock there were dockers \
                    there were dogs and cats \
                    and it was raining cats and dogs
                    a dog and a cat were on both sides of the rain"
            .split_ascii_whitespace()
            .collect::<Vec<_>>();

        assert_eq!(solution(&words), word_frequency_counter(&words));
    }

    #[test]
    fn test_empty() {
        assert_eq!(solution(&[]), word_frequency_counter(&[]));
    }

    #[test]
    fn test_only_repeated() {
        let words = std::iter::repeat_n("one", 9).collect::<Vec<_>>();

        assert_eq!(solution(&words), word_frequency_counter(&words));
    }
}
