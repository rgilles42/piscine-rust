use tic_tac_toe::*;

fn main() {
    println!(
        "{:?}",
        tic_tac_toe(vec![
            vec!["O", "X", "O"],
            vec!["O", "P", "X"],
            vec!["X", "#", "X"]
        ])
    );
    // "tie"
    println!(
        "{:?}",
        tic_tac_toe(vec![
            vec!["X", "O", "O"],
            vec!["X", "O", "O"],
            vec!["#", "O", "X"]
        ])
    );
    // "player O won"

    let dig = vec![
        vec!["O", "O", "X"],
        vec!["O", "X", "O"],
        vec!["X", "#", "X"],
    ];

    println!("{:?}", tic_tac_toe(dig));
    // "player X won"
}
#[cfg(test)]
mod tests {
    use super::*;

    struct Test<'a> {
        player: &'a str,
        table: Vec<Vec<&'a str>>,
        result: &'a str,
    }

    impl Test<'_> {
        fn init_horizontal() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "O", "O"],
                        vec!["X", "X", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["X", "X", "O"],
                        vec!["O", "O", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "#", "O"],
                        vec!["X", "X", "X"],
                    ],
                    result: "player X won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "X", "X", "X"],
                        vec!["X", "#", "O", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "player X won",
                },
            ]
        }
        fn init_tie() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "tie",
                },
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["X", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "tie",
                },
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "O", "X", "X"],
                        vec!["X", "#", "X", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "tie",
                },
            ]
        }
        fn init_vertical() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "X", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["X", "O", "O"],
                        vec!["X", "O", "O"],
                        vec!["#", "O", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "X"],
                        vec!["O", "X", "X"],
                        vec!["X", "#", "X"],
                    ],
                    result: "player X won",
                },
            ]
        }
        fn init_diagonals() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "O", "X"],
                        vec!["O", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "player X won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["X", "O", "O"],
                        vec!["X", "#", "O"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "O", "X", "O"],
                        vec!["X", "#", "O", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "player O won",
                },
            ]
        }
    }

    #[test]
    fn test_diagonals() {
        let new_tests = Test::init_diagonals();
        for v in new_tests {
            assert_eq!(diagonals(v.player, &v.table), true)
        }

        assert_eq!(
            diagonals(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "X", "X"],
                    vec!["X", "#", "X"]
                ]
            ),
            false
        );
    }

    #[test]
    fn test_horizontal() {
        let new_tests = Test::init_horizontal();
        for v in new_tests {
            assert_eq!(horizontal(v.player, &v.table), true)
        }

        assert_eq!(
            horizontal(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "O", "X"],
                    vec!["X", "#", "O"]
                ]
            ),
            false
        );
    }

    #[test]
    fn test_vertical() {
        let new_tests = Test::init_vertical();
        for v in new_tests {
            assert_eq!(vertical(v.player, &v.table), true)
        }

        assert_eq!(
            vertical(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "O", "X"],
                    vec!["X", "#", "O"]
                ]
            ),
            false
        );
    }

    #[test]
    fn test_tic_tac_toe() {
        let mut new_tests = Test::init_diagonals();
        new_tests.append(&mut Test::init_horizontal());
        new_tests.append(&mut Test::init_vertical());
        new_tests.append(&mut Test::init_tie());

        for v in new_tests {
            assert_eq!(tic_tac_toe(v.table), v.result.to_string());
        }
    }
}
