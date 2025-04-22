use armstrong_number::*;

fn main() {
    println!("{:?}", is_armstrong_number(0));
    println!("{:?}", is_armstrong_number(1));
    println!("{:?}", is_armstrong_number(153));
    println!("{:?}", is_armstrong_number(370));
    println!("{:?}", is_armstrong_number(371));
    println!("{:?}", is_armstrong_number(407));
    println!("{:?}", is_armstrong_number(400));
    println!("{:?}", is_armstrong_number(198));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_examples() {
        assert_eq!(Some(0), is_armstrong_number(0));
        assert_eq!(Some(1), is_armstrong_number(1));
        assert_eq!(Some(153), is_armstrong_number(153));
        assert_eq!(Some(370), is_armstrong_number(370));
        assert_eq!(Some(371), is_armstrong_number(371));
        assert_eq!(Some(407), is_armstrong_number(407));
        assert_eq!(None, is_armstrong_number(400));
        assert_eq!(None, is_armstrong_number(198));
    }

    #[test]
    fn test_valid_numbers() {
        assert_eq!(Some(2), is_armstrong_number(2));
        assert_eq!(Some(3), is_armstrong_number(3));
        assert_eq!(Some(4), is_armstrong_number(4));
        assert_eq!(Some(5), is_armstrong_number(5));
        assert_eq!(Some(6), is_armstrong_number(6));
        assert_eq!(Some(7), is_armstrong_number(7));
        assert_eq!(Some(8), is_armstrong_number(8));
        assert_eq!(Some(9), is_armstrong_number(9));
        assert_eq!(Some(1634), is_armstrong_number(1634));
        assert_eq!(Some(8208), is_armstrong_number(8208));
        assert_eq!(Some(9474), is_armstrong_number(9474));
        assert_eq!(Some(54748), is_armstrong_number(54748));
        assert_eq!(Some(92727), is_armstrong_number(92727));
        assert_eq!(Some(93084), is_armstrong_number(93084));
    }

    #[test]
    fn test_invalid_numbers() {
        assert_eq!(None, is_armstrong_number(11));
        assert_eq!(None, is_armstrong_number(98));
        assert_eq!(None, is_armstrong_number(100));
        assert_eq!(None, is_armstrong_number(90438));
        assert_eq!(None, is_armstrong_number(940));
        assert_eq!(None, is_armstrong_number(85));
    }
}
