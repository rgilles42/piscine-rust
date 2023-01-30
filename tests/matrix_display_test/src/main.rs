// Implement the std::fmt::Display trait for a matrix of i32
// using the struct Matrix define the associated function new that
// creates a new Matrix from &[&[i32]]
// After implement the std::fmt::Display trait to print the matrix
// like this

// ```
// (1 2 3)
// (4 5 6)
// (7 8 9)
// ```

use matrix_display::*;

fn main() {
    let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    println!("{}", matrix);
}

#[cfg(test)]
mod tests {
    use lib::{Kind, TestProperties};
    use matrix_display::*;

    #[test]
    fn it_works() {
        let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
        let display = String::from("(1 2 3)\n(4 5 6)\n(7 8 9)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_matrix_col() {
        let matrix = Matrix::new(&[&[1], &[2], &[3]]);
        let display = String::from("(1)\n(2)\n(3)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_matrix_row() {
        let matrix = Matrix::new(&[&[1, 2, 3]]);
        let display = String::from("(1 2 3)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_m_by_n_matrix() {
        let matrix = Matrix::new(&[&[1, 2, 3, 4, 5], &[6, 7, 8, 9, 10], &[11, 12, 13, 14, 15]]);
        let display = String::from("(1 2 3 4 5)\n(6 7 8 9 10)\n(11 12 13 14 15)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_empty_matrix() {
        let matrix = Matrix::new(&[&[]]);
        let display = String::from("()");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }
}
