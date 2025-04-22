use matrix_transposition::*;

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}

#[cfg(test)]
mod tests {
    use matrix_transposition::*;

    #[test]
    fn transpose_zero() {
        let m = Matrix((0, 0), (0, 0));
        let m = transpose(m);
        assert_eq!(m, Matrix((0, 0), (0, 0)));
    }

    #[test]
    fn transpose_identity() {
        let m = Matrix((1, 0), (0, 1));
        let m = transpose(m);
        assert_eq!(m, Matrix((1, 0), (0, 1)));
    }

    #[test]
    fn transpose_other_cases() {
        let m = Matrix((1, 3), (4, 5));
        let m = transpose(m);
        assert_eq!(m, Matrix((1, 4), (3, 5)));

        let m = Matrix((6, 80), (12, 3));
        let m = transpose(m);
        assert_eq!(m, Matrix((6, 12), (80, 3)));
    }
}
