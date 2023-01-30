// Define a function that calculate the transpose matrix of a 4x3 (4
// rows by 3 columns) matrix which is a 3x4 matrix (3 rows by 4 columns)

// You don't need to understand everything about matrices

// Just convert lines into columns and vice versa
// ( a b )	__ transposition __> ( a d )
// ( c d )  					 ( b d )

// Only the body of the transpose function can be changed

// fn main() {
// 	let matrix = Matrix((1, 3), (4, 5));
// 	println!("Original matrix {:?}", matrix);
// 	println!("Transpose matrix {:?}", transpose(matrix));
// }

use matrix_transposition_4by3::{transpose, Matrix4by3};

fn main() {
    let matrix = Matrix4by3((1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}

#[cfg(test)]
mod tests {
    use matrix_transposition_4by3::*;

    #[test]
    fn test_tranposion() {
        let matrix = Matrix4by3((1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12));
        let expected = Matrix3by4((1, 4, 7, 10), (2, 5, 8, 11), (3, 6, 9, 12));
        assert_eq!(transpose(matrix), expected);
    }
}
