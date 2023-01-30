use negative_spelling::*;

fn main() {
    println!("{}", negative_spell(-1234));
    println!("{}", negative_spell(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_numbers() {
        assert_eq!(negative_spell(0), String::from("zero"));
        assert_eq!(negative_spell(-1), String::from("minus one"));
        assert_eq!(negative_spell(-14), String::from("minus fourteen"));
        assert_eq!(negative_spell(-20), String::from("minus twenty"));
        assert_eq!(negative_spell(-22), String::from("minus twenty-two"));
        assert_eq!(negative_spell(-101), String::from("minus one hundred one"));
        assert_eq!(
            negative_spell(-120),
            String::from("minus one hundred twenty")
        );
        assert_eq!(
            negative_spell(-123),
            String::from("minus one hundred twenty-three")
        );
    }
    #[test]
    fn test_medium_numbers() {
        assert_eq!(negative_spell(-1000), String::from("minus one thousand"));
        assert_eq!(
            negative_spell(-1055),
            String::from("minus one thousand fifty-five")
        );
        assert_eq!(
            negative_spell(-1234),
            String::from("minus one thousand two hundred thirty-four")
        );
        assert_eq!(
            negative_spell(-10123),
            String::from("minus ten thousand one hundred twenty-three")
        );
    }
    #[test]
    fn test_long_numbers() {
        assert_eq!(
            negative_spell(-910112),
            String::from("minus nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            negative_spell(-651123),
            String::from("minus six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(
            negative_spell(-810000),
            String::from("minus eight hundred ten thousand")
        );
        assert_eq!(negative_spell(-1000000), String::from("minus one million"));
    }
    #[test]
    fn test_invalid_numbers() {
        assert_eq!(negative_spell(1), String::from("error: positive number"));
        assert_eq!(
            negative_spell(2390904),
            String::from("error: positive number")
        );
    }
}
