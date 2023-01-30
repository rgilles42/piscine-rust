use modify_letter::*;

fn main() {
    println!("{}", remove_letter_sensitive("hEey hEey", 'e'));
    println!("{}", remove_letter_insensitive("hEye", 'e'));
    println!("{}", swap_letter_case("BbBb", 'b'));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_letter_sensitive() {
        assert_eq!(
            remove_letter_sensitive("Jijm jis mijssjing", 'j'),
            "Jim is missing"
        );
        assert_eq!(
            remove_letter_sensitive("Jack is missing", 'j'),
            "Jack is missing"
        );
        assert_eq!(
            remove_letter_sensitive("Jjjijll jis mijssjjing", 'j'),
            "Jill is missing"
        );
    }

    #[test]
    fn test_remove_letter_insensitive() {
        assert_eq!(
            remove_letter_insensitive("JaillA ais swiaAmmingA", 'A'),
            "Jill is swimming"
        );
        assert_eq!(
            remove_letter_insensitive("Jim is swimming", 'A'),
            "Jim is swimming"
        );
        assert_eq!(
            remove_letter_insensitive("JoaseA ais swiaAAAmmingA", 'A'),
            "Jose is swimming"
        );
    }

    #[test]
    fn test_swap_letter_case() {
        assert_eq!(swap_letter_case("AaBbCcDdEe", 'e'), "AaBbCcDdeE");
        assert_eq!(swap_letter_case("AaBbCcDd", 'e'), "AaBbCcDd");
        assert_eq!(
            swap_letter_case("AaBbCcDdEEEeeeEeEe", 'e'),
            "AaBbCcDdeeeEEEeEeE"
        );
    }

    #[test]
    fn test_empty_arguments() {
        assert_eq!(remove_letter_sensitive("", 'a'), "");
        assert_eq!(remove_letter_insensitive("", 'b'), "");
        assert_eq!(swap_letter_case("", 'c'), "");
    }
}
