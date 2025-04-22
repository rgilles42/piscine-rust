use cipher::*;

#[test]
fn test_ok_values() {
    assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Ok(()));
    assert_eq!(cipher("asdasd", "zhwzhw"), Ok(()));
    assert_eq!(cipher("3(/&%fsd 32das", "3(/&%uhw 32wzh"), Ok(()));
}

#[test]
fn test_empty_values() {
    assert_eq!(cipher("", ""), Ok(()));
    assert_eq!(
        cipher("", "1Svool 2dliow!"),
        Err(CipherError {
            expected: "".to_owned()
        })
    );
    assert_eq!(
        cipher("1Hello 2world!", ""),
        Err(CipherError {
            expected: "1Svool 2dliow!".to_owned()
        })
    );
}

#[test]
fn test_errors() {
    assert_eq!(
        cipher("1Hello 2world!", "1svool 2dliow!"),
        Err(CipherError {
            expected: String::from("1Svool 2dliow!")
        })
    );
    assert_eq!(
        cipher("asdasd", "lkdas"),
        Err(CipherError {
            expected: String::from("zhwzhw")
        })
    );
    assert_eq!(
        cipher("3(/&%sd 32das", "3(/&%uhw 32wzh"),
        Err(CipherError {
            expected: String::from("3(/&%hw 32wzh")
        })
    );
}
