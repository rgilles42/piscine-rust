use invert_sentence::*;

#[test]
fn empty_sentence() {
    assert_eq!(invert_sentence(""), "");
}

#[test]
fn single_word_sentence() {
    assert_eq!(invert_sentence("word"), "word");
}

#[test]
fn multiple_words_sentence() {
    assert_eq!(invert_sentence("Rust is Awesome"), "Awesome is Rust");
}

#[test]
fn multiple_leading_and_trailing_whitespaces() {
    assert_eq!(
        invert_sentence("    word1     word2  "),
        "    word2     word1  "
    );
}

#[test]
fn multiple_leading_and_trailing_whitespaces_repeated_words() {
    assert_eq!(
        invert_sentence("    word1     word2    word1     "),
        "    word1     word2    word1     "
    );

    assert_eq!(
        invert_sentence("    word1     word2    word3     "),
        "    word3     word2    word1     "
    );
}

#[test]
fn multiple_words_sentence_with_punctuation() {
    let sentence = "Hello, World!";
    assert_eq!(invert_sentence(sentence), "World! Hello,");
}

#[test]
fn complex_example() {
    assert_eq!(
        invert_sentence("\tI'm  alive\t\t!\t \t"),
        "\t!  alive\t\tI'm\t \t"
    );
}
