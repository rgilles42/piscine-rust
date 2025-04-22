use reverse_string::*;

fn main() {
    println!("{}", rev_str("Hello, world!"));
    println!("{}", rev_str("Hello, my name is Roman"));
    println!("{}", rev_str("I have a nice car!"));
    println!("{}", rev_str("How old are You"));
    println!("{}", rev_str("ex: this is an example água"));
}

#[cfg(test)]
mod tests {
    use reverse_string::*;

    #[test]
    fn test_one_word() {
        assert_eq!(rev_str("robot"), "tobor");
        assert_eq!(rev_str("Ramen"), "nemaR");
        assert_eq!(rev_str("racecar"), "racecar");
        assert_eq!(rev_str("drawer"), "reward");
        assert_eq!(rev_str("子猫"), "猫子");
        assert_eq!(rev_str(""), "");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(rev_str("I'm hungry!"), "!yrgnuh m'I");
        assert_eq!(rev_str("Hello, world!"), "!dlrow ,olleH");
        assert_eq!(
            rev_str("Hello, my name is Roman"),
            "namoR si eman ym ,olleH"
        );
        assert_eq!(rev_str("I have a nice car!"), "!rac ecin a evah I");
        assert_eq!(rev_str("How old are You"), "uoY era dlo woH");
        assert_eq!(
            rev_str("ex: this is an example água"),
            "augá elpmaxe na si siht :xe"
        );
    }
}
