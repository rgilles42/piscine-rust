use minesweeper::*;

fn main() {
    println!("{:?}", solve_board(&[]));
    println!("{:?}", solve_board(&[""]));
    println!("{:?}", solve_board(&["***"]));
    println!("{:#?}", solve_board(&["   ", " * ", "   ",]));
    println!("{:#?}", solve_board(&["*  ", "   ", "  *",]));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(expected: &[&str]) {
        let expected: Vec<String> = expected.iter().map(|row| row.chars().collect()).collect();
        let to_solve = get_to_solve(&expected);
        let to_solve_strs = to_solve.iter().map(|row| row.as_str()).collect::<Vec<_>>();
        assert_eq!(expected, solve_board(&to_solve_strs))
    }

    fn get_to_solve(expected: &[String]) -> Vec<String> {
        expected
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| if c == '*' { c } else { ' ' })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_subject_example() {
        #[rustfmt::skip]
        run_test(&[
            "111",
            "1*1",
            "111",
        ]);
    }

    #[test]
    fn test_no_rows() {
        #[rustfmt::skip]
        run_test(&[
        ]);
    }

    #[test]
    fn test_one_char() {
        #[rustfmt::skip]
        run_test(&[
            "*",
        ]);
    }

    #[test]
    fn test_only_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "***",
            "***",
        ]);
    }

    #[test]
    fn test_no_mines() {
        #[rustfmt::skip]
        run_test(&[
            "   ",
            "   ",
            "   ",
        ]);
    }

    #[test]
    fn test_no_columns() {
        #[rustfmt::skip]
        run_test(&[
            "",
        ]);
    }

    #[test]
    fn test_space_in_middle() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "*8*",
            "***",
        ]);
    }

    #[test]
    fn test_complex_case() {
        #[rustfmt::skip]
        run_test(&[
            " 2**211",
            "13*43*1",
            "*334*32",
            "12**22*",
        ]);
    }
}
