fn main() {}

#[cfg(test)]
mod tests {
    use simple_hash::*;
    use std::collections::HashMap;

    fn reference_hash(words: Vec<&str>) -> HashMap<&str, usize> {
        let mut frequency_count: HashMap<&str, usize> = HashMap::new();

        for word in words {
            *frequency_count.entry(word).or_insert(0) += 1;
        }
        frequency_count
    }

    #[test]
    fn test_basic_example() {
        let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests"
            .to_string();
        let words = sentence.split(" ").collect::<Vec<&str>>();
        assert_eq!(
            reference_hash(words.clone()),
            word_frequency_counter(words.clone())
        );
    }

    #[test]
    fn test_frequency_counter() {
        let sentence = "on the dock there were dockers \
                        there were dogs and cats \
                        and it was raining cats and dogs
                        a dog and a cat were on both sides of the rain"
            .to_string();
        let words = sentence.split(" ").collect::<Vec<&str>>();
        assert_eq!(
            reference_hash(words.clone()),
            word_frequency_counter(words.clone())
        );
    }

    #[test]
    fn test_empty() {
        let words = Vec::<&str>::new();
        assert_eq!(
            reference_hash(words.clone()),
            word_frequency_counter(words.clone())
        );
    }

    #[test]
    fn test_only_repeated() {
        let sentence = "one one one one one one one one one".to_string();
        let words = sentence.split(" ").collect::<Vec<&str>>();
        assert_eq!(
            reference_hash(words.clone()),
            word_frequency_counter(words.clone())
        );
    }
}
