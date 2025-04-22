use capitalizing::*;

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}", change_case("heLLo THere"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("this is working"), "This is working");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("this is a title"), "This Is A Title");
        assert_eq!(
            title_case("hello my\t\tname is carl"),
            "Hello My\t\tName Is Carl"
        );
    }

    #[test]
    fn test_change_case() {
        assert_eq!(change_case("PROgraming"), "proGRAMING");
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
        assert_eq!(title_case(""), "");
        assert_eq!(change_case(""), "");
    }
}
