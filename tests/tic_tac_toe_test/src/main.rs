use tic_tac_toe::*;

fn main() {
    println!(
        "{}",
        tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
    );
    // tie
    println!(
        "{}",
        tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
    );
    // player O won

    let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

    println!("{}", tic_tac_toe(diag));
    // player X won
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIAGONAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('X', [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']]),
        ('O', [['O', 'X', 'O'], ['X', 'O', 'O'], ['X', '#', 'O']]),
    ];

    const HORIZONTAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('O', [['O', 'O', 'O'], ['X', 'X', 'O'], ['O', '#', 'X']]),
        ('O', [['X', 'X', 'O'], ['O', 'O', 'O'], ['O', '#', 'X']]),
        ('X', [['O', 'X', 'O'], ['O', '#', 'O'], ['X', 'X', 'X']]),
    ];

    const VERTICAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('O', [['O', 'X', 'O'], ['O', 'X', 'O'], ['O', '#', 'X']]),
        ('O', [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']]),
        ('X', [['O', 'X', 'X'], ['O', 'X', 'X'], ['X', '#', 'X']]),
    ];

    const TIE_TESTS: &[[[char; 3]; 3]] = &[
        [['O', 'X', 'O'], ['O', 'X', 'O'], ['X', '#', 'X']],
        [['O', 'X', 'O'], ['X', 'X', 'O'], ['X', '#', 'X']],
    ];

    #[test]
    fn test_diagonal() {
        DIAGONAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(diagonals(p, t)));
    }

    #[test]
    fn test_horizontal() {
        HORIZONTAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(horizontal(p, t)));
    }

    #[test]
    fn test_vertical() {
        VERTICAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(vertical(p, t)));
    }

    #[test]
    fn test_tic_tac_toe() {
        [DIAGONAL_TESTS, HORIZONTAL_TESTS, VERTICAL_TESTS]
            .concat()
            .into_iter()
            .for_each(|(p, t)| assert_eq!(tic_tac_toe(t), format!("player {} won", p)));

        TIE_TESTS
            .iter()
            .copied()
            .for_each(|t| assert_eq!(tic_tac_toe(t), "tie"));
    }
}
