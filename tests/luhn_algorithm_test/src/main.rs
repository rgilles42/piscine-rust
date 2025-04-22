use luhn_algorithm::*;

fn main() {
    println!("{}", is_luhn_formula(""));
    println!("{}", is_luhn_formula("1"));
    println!("{}", is_luhn_formula("79927398713"));
    println!("{}", is_luhn_formula("7992 7398 713"));
    println!("{}", is_luhn_formula("1234567890123456"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_examples() {
        assert!(is_luhn_formula("") == false, "An empty string is not valid");
        assert!(is_luhn_formula("1") == false, "1 is not valid");
        assert!(is_luhn_formula("79927398713") == true, "79927398713 is valid");
    }

    #[test]
    fn test_valid_numbers() {
        assert!(is_luhn_formula("4532015112830366") == true, "4532015112830366 is valid");
        assert!(is_luhn_formula("6011 1111 1111 1117") == true, "6011 1111 1111 1117 is valid");
        assert!(is_luhn_formula("371449635398431") == true, "371449635398431 is valid");
        assert!(is_luhn_formula("  5555555555554444") == true, "  5555555555554444 is valid");
        assert!(is_luhn_formula("79927398713 8336263  ") == true, "79927398713 8336263   is valid");
    }

    #[test]
    fn test_invalid_numbers() {
        assert!(is_luhn_formula("1234567890123456") == false, "1234567890123456 is not valid");
        assert!(is_luhn_formula("a6011 1111 1111 1117x") == false, "a6011 1111 1111 1117x is not valid");
        assert!(is_luhn_formula("4444333322221110") == false, "4444333322221110 is not valid");
        assert!(is_luhn_formula("9876543210987654") == false, "9876543210987654 is not valid");
        assert!(is_luhn_formula("8765432198765432") == false, "8765432198765432 is not valid");
        assert!(is_luhn_formula("1111222233334445") == false, "1111222233334445 is not valid");
    }
}
