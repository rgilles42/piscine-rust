use scytale_cipher::*;

fn main() {
    println!("{:?}", scytale_cipher(String::from("attack morning"), 6));
    // output : a ntmgto ar cn ki
}

#[test]
fn test_scytale_cipher() {
    scytale_cipher(String::from("attack morning"), 6);
    assert_eq!(
        &scytale_cipher(String::from("scytale Code"), 6),
        "sec yCtoadle"
    );
    assert_eq!(
        &scytale_cipher(String::from("scytale Code"), 8),
        "sCcoydtea l e"
    );
    // nothing
    assert_eq!(&scytale_cipher(String::from(""), 4), "");
    // same len
    assert_eq!(
        &scytale_cipher(String::from("qwerty qwerty"), 13),
        "qwerty qwerty"
    );
    // different cylinder
    assert_eq!(
        &scytale_cipher(String::from("attack morning"), 6),
        "a ntmgto ar cn ki"
    );
}
