/*
## expected_variable

### Instructions

Create a **function** named `expected_variable` that receives a string to compare and an expected string. It should return an `Option`. Every comparison should be case `insensitive`.

If the compared string is **not** in camel case or snake case, `expected_variable` returns `None`. You can use the `case` crate to figure that out. Otherwise, the compared string should be compared to the expected string using the `edit_distance` **function** that you have already created.

If the result of `edit_distance` has more than 50% alikeness to the expected string, considering the length of the expected string and the result of `edit_distance`, the function should return that value with a `'%'` symbol after the number.
Otherwise `expected_value` should return `None`.

Example:

```rs
fn main() {
    println!(
        "{} close to it",
        expected_variable("On_Point", "on_point").unwrap()
    ); // -> 100%
    println!(
        "{} close to it",
        expected_variable("soClose", "So_Close").unwrap()
    ); // -> 88%
    println!(
        "{} close to it",
        expected_variable("something", "something_completely_different").unwrap()
    ); // -> Fail
    println!(
        "{} close to it",
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
    ); // -> 73%
}
```

*/

#[cfg(test)]
mod tests {

    use expected_variable::*;

    #[test]
    fn no_variable_case() {
        let mut result = expected_variable("It is simply not a variable case", "gonnaFail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("do-not-use-dashes", "do-not-use-dashes");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("Not a variable case", "needs to fail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("This should be None", "needs to fail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("Do not use spaces", "Do not use spaces");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );
    }

    #[test]
    fn incorrect_names() {
        let mut result = expected_variable("it_is_done", "almost_there");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("frankenstein", "Dracula");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );
    }

    #[test]
    fn one_hundred_percent() {
        assert_eq!(
            "100%",
            expected_variable("great_variable", "great_variable").unwrap()
        );
        assert_eq!("100%", expected_variable("SpOtON", "SpotOn").unwrap());
        assert_eq!(
            "100%",
            expected_variable("Another_great_variable", "Another_great_variable").unwrap()
        );
    }

    #[test]
    fn passing_tests() {
        assert_eq!("88%", expected_variable("soClose", "So_Close").unwrap());
        assert_eq!("73%", expected_variable("lets_try", "lets_try_it").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!(
            "67%",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}
