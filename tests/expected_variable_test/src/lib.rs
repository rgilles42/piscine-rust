use expected_variable::*;

#[test]
fn regular_cases() {
    assert_eq!(
        Some("88%".to_owned()),
        expected_variable("soClose", "so_close")
    );
    assert_eq!(
        Some("73%".to_owned()),
        expected_variable("lets_try", "lets_try_it")
    );
    assert_eq!(
        Some("64%".to_owned()),
        expected_variable("GoodJob", "VeryGoodJob")
    );
    assert_eq!(
        Some("67%".to_owned()),
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch")
    );
}

#[test]
fn none_cases() {
    assert_eq!(
        None,
        expected_variable("It is simply not a variable case", "gonnaFail")
    );

    assert_eq!(
        None,
        expected_variable("do-not-use-dashes", "do-not-use-dashes")
    );

    assert_eq!(
        None,
        expected_variable("Not a variable case", "needs to fail")
    );

    assert_eq!(
        None,
        expected_variable("This should be None", "needs to fail")
    );

    assert_eq!(
        None,
        expected_variable("Do not use spaces", "Do not use spaces")
    );
}

#[test]
fn incorrect_cases() {
    assert_eq!(None, expected_variable("it_is_done", "almost_there"));

    assert_eq!(None, expected_variable("frankenstein", "Dracula"));
}

#[test]
fn equal_cases() {
    assert_eq!(
        Some("100%".to_owned()),
        expected_variable("great_variable", "great_variable")
    );
    assert_eq!(
        Some("100%".to_owned()),
        expected_variable("SpOtON", "SpotOn")
    );
    assert_eq!(
        Some("100%".to_owned()),
        expected_variable("Another_great_variable", "Another_great_variAble")
    );
}
