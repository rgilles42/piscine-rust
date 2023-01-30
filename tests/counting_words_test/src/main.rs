/*
## counting_words

### Instructions

In this program you will have to create a function `counting_words` that
receives a `&str` and returns each word and the number of times it appears on the string.

The program will count as a word the following:

- A number like ("0" or "1234") will count as 1.
- A simple word or letter like ("a" or "they") will count as 1.
- Two simple words joined by a single apostrophe ("it's" or "they're") will count as 1.

The program must respect the following rules:

- The count is case insensitive ("HELLO", "Hello", and "hello") are 3 uses of the same word.
- All forms of punctuation have to be ignored except for the apostrophe if used like the example above.
- The words can be separated by any form of whitespace (ie "\t", "\n", " ").

### Expected Function

```rust
fn counting_words(words: &str) -> HashMap<String, u32> {}
```

### Usage

Here is a possible program to test your function :

```rust
use counting_words::counting_words;
use std::collections::HashMap;

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}
```

And its output:

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
{"hello": 1, "world": 1}
{"and": 2, "human": 1, "universe": 2, "the": 2, "i\'m": 1, "about": 1, "einstein": 1, "are": 1, "infinite": 1, "sure": 1, "albert": 1, "two": 1, "things": 1, "not": 1, "stupidity": 1}
{"batman": 3, "stop": 2}
student@ubuntu:~/[[ROOT]]/test$
```
*/

use counting_words::*;
use std::collections::HashMap;

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    fn test_counting(input: &str, expected: &[(&str, u32)]) {
        let mut m: HashMap<String, u32> = counting_words(input);
        for &(k, v) in expected.iter() {
            assert_eq!(
                (k, m.remove(&k.to_string().to_lowercase()).unwrap_or(0)),
                (k, v)
            );
        }
        // may fail with a message that clearly shows all extra pairs in the map
        assert_eq!(m.iter().collect::<Vec<(&String, &u32)>>(), vec![]);
    }
    #[test]
    fn test_simple() {
        test_counting("word", &[("word", 1)]);
        test_counting("hello", &[("hello", 1)]);
        test_counting("hello big world", &[("hello", 1), ("big", 1), ("world", 1)]);
        test_counting("one of each", &[("one", 1), ("of", 1), ("each", 1)]);
        test_counting("Hello, 1, 2 HELLO", &[("Hello", 2), ("1", 1), ("2", 1)]);
        test_counting(
            "Batman, BATMAN, batman, Stop stop",
            &[("batman", 3), ("stop", 2)],
        );
        test_counting(
            " multiple   whitespace",
            &[("multiple", 1), ("whitespace", 1)],
        );
    }

    #[test]
    fn test_count_multiple_occurrences() {
        test_counting(
            "one fish two fish red fish blue fish",
            &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)],
        );
    }

    #[test]
    fn test_multi_lines() {
        test_counting(
            "Game\nNight\nTomorrow",
            &[("Game", 1), ("Night", 1), ("Tomorrow", 1)],
        );
    }

    #[test]
    fn test_punctuation() {
        test_counting(
            "keyboard : mouse on the desk : Computer!!&@$%^&",
            &[
                ("keyboard", 1),
                ("mouse", 1),
                ("on", 1),
                ("the", 1),
                ("desk", 1),
                ("Computer", 1),
            ],
        );
    }

    #[test]
    fn with_apostrophes() {
        test_counting(
            "First: don't laugh. Then: don't cry.",
            &[
                ("first", 1),
                ("don't", 2),
                ("laugh", 1),
                ("then", 1),
                ("cry", 1),
            ],
        );
    }

    #[test]
    fn test_apostrophe() {
        test_counting(
            "Joe can't tell between 'large' and large.",
            &[
                ("joe", 1),
                ("can't", 1),
                ("tell", 1),
                ("between", 1),
                ("large", 2),
                ("and", 1),
            ],
        );
    }
}
