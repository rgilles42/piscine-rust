// Define a function call thirtytwo_tens that returns an array with 32
// positions fill with only the value 10: [10, 10, 10, ... 10].len()
// = 32

// Write a function that takes an array of i32 and returns the sum of
// the elements (make it work with the main)
use arrays::*;

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a1: Vec<i32> = (1..11).collect();
    let b = [5; 10];

    println!("The Sum of the elements in {:?} = {}", a, sum(&a));
    println!("The Sum of the elements in {:?} = {}", a1, sum(&a1));
    println!("The Sum of the elements in {:?} = {}", b, sum(&b));
    println!(
        "Array size {} with only 10's in it {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::{Kind, TestProperties};

    #[test]
    fn test_thirtytwo_tens() {
        let test = TestProperties {
            name: "thirtytwo_tens",
            kind: Kind::Function,
        };
        test.assert_with_message(&[], thirtytwo_tens(), [10; 32]);
    }

    #[test]
    fn test_sum() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let test = TestProperties {
            kind: Kind::Function,
            name: "sum",
        };
        test.assert_with_message(&[Box::new(a)], sum(&a), a.iter().sum());
    }
}
