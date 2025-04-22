#[test]
fn test_error_ms() {
    ["", "stupid", "you are stupid"]
        .into_iter()
        .for_each(|m| assert_eq!(Err("ERROR: illegal"), profanity_filter::check_ms(m)));
}

#[test]
fn test_ok_ms() {
    [
        "get out of the car",
        "no!",
        "get the werewolf",
        "wait the what...",
    ]
    .into_iter()
    .for_each(|m| assert_eq!(Ok(m), profanity_filter::check_ms(m)));
}
