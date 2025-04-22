use ownership::first_subword;

fn main() {
    let s1 = "helloWorld";
    let s2 = "snake_case";
    let s3 = "CamelCase";
    let s4 = "just";

    println!("first_subword({}) = {}", s1, first_subword(s1.to_owned()));
    println!("first_subword({}) = {}", s2, first_subword(s2.to_owned()));
    println!("first_subword({}) = {}", s3, first_subword(s3.to_owned()));
    println!("first_subword({}) = {}", s4, first_subword(s4.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_subword_test() {
        let cases = [
            ("helloWorld", "hello"),
            ("helloBigWorld", "hello"),
            ("how_are_you", "how"),
            ("Changeyou", "Changeyou"),
            ("CamelCase", "Camel"),
        ];

        cases
            .into_iter()
            .for_each(|(i, e)| assert_eq!(first_subword(i.to_owned()), e));
    }
}
