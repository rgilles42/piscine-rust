use easy_traits::*;

fn main() {
    let mut str_aux = StringValue {
        value: String::from("hello"),
    };

    println!("Before append: {}", str_aux.value);

    str_aux.append_str(String::from(" there!"));
    println!("After append: {}", str_aux.value);

    str_aux.remove_punctuation_marks();
    println!("After removing punctuation: {}", str_aux.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_str() {
        let mut str_aux = StringValue {
            value: String::from("hello"),
        };

        assert_eq!(
            String::from("hello there!"),
            str_aux.append_str(String::from(" there!")).value
        );

        assert_eq!(
            String::from("hello there! How are You?"),
            str_aux.append_str(String::from(" How are You?")).value
        );

        assert_eq!(
            String::from("hello there How are You"),
            str_aux.remove_punctuation_marks().value
        );
    }

    #[test]
    fn test_remove_punctuation() {
        let mut str_aux = StringValue {
            value: String::from("!?.,!?.,"),
        };

        assert_eq!(String::from(""), str_aux.remove_punctuation_marks().value);

        assert_eq!(
            String::from("h!e!l?lo. the,.re!"),
            str_aux.append_str(String::from("h!e!l?lo. the,.re!")).value
        );
        assert_eq!(
            String::from("hello there"),
            str_aux.remove_punctuation_marks().value
        );
    }

    #[test]
    fn test_append_number() {
        let mut str_aux = StringValue {
            value: String::from(""),
        };

        assert_eq!(String::from("-1"), str_aux.append_number(-1.0).value);

        assert_eq!(String::from("-15"), str_aux.append_number(5.0).value);

        assert_eq!(String::from("-155.5"), str_aux.append_number(5.5).value);

        assert_eq!(
            String::from("-1555"),
            str_aux.remove_punctuation_marks().value
        );
    }
}
