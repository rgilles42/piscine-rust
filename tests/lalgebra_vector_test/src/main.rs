// A vector in linear algebra is define as "anything that can be added
// and that can be multiplied by a scalar"
// And the associated function dot that calculates the dot product
// between two vectors
// let vector = Vector(vec![0,3, 4]);
// let vector_1 = Vector(vec![0,3,3]);
// vector.dot(&vector_1) == Some(23);

// The dot product between two vectors of different length it's not defined

use lalgebra_vector::Vector;

fn main() {
    let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
    let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
    println!("{:?}", vector_1.dot(&vector_2));
    println!("{:?}", vector_1 + vector_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::{Kind, TestProperties};

    #[test]
    fn dot_product() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        let meta_test = TestProperties {
            name: "dot",
            kind: Kind::Method,
        };
        let expected: i64 = 3;
        meta_test.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1.dot(&vector_2),
            Some(expected),
        );

        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2]);
        meta_test.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1.dot(&vector_2),
            None,
        );
    }

    #[test]
    fn addition() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        let test_meta = TestProperties {
            name: "+",
            kind: Kind::Operator,
        };
        test_meta.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1 + vector_2,
            Some(Vector(vec![5i64, 1, -6])),
        );

        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![2, 4, -2, -1]);
        test_meta.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1 + vector_2,
            None,
        );
    }
}
