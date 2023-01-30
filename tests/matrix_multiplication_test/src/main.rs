use matrix_multiplication::{multiply, Matrix};

fn main() {
    let matrix = Matrix((1, 2), (3, 4));
    let val = 2;
    println!("Original matrix {:?}", matrix);
    println!("Matrix after multiply {:?}", multiply(matrix, val));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiply_1() {
        let m = Matrix((1, 2), (3, 4));
        let val = 5;
        assert_eq!(multiply(m, val), Matrix((5, 10), (15, 20)));
    }
    #[test]
    fn test_multiply_2() {
        let m = Matrix((1, 2), (3, 4));
        let val = -5;
        assert_eq!(multiply(m, val), Matrix((-5, -10), (-15, -20)));
    }
    #[test]
    fn test_multiply_3() {
        let m = Matrix((1, 2), (3, 4));
        let val = 0;
        assert_eq!(multiply(m, val), Matrix((0, 0), (0, 0)));
    }
}
