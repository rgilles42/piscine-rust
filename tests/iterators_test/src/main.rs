use iterators::Collatz;
use iterators::*;

fn main() {
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(12));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_seven() {
        let test_value = vec![1, 2, 3, 4, 5, 6, 7];
        let test_result = vec![0, 1, 7, 2, 5, 8, 16];

        for i in 0..test_value.len() {
            assert_eq!(test_result[i], collatz(test_value[i]));
        }
    }

    #[test]
    fn test_big_numbers() {
        let test_value = vec![54, 888, 4372, 9999];
        let test_result = vec![112, 72, 33, 91];

        for i in 0..test_value.len() {
            assert_eq!(test_result[i], collatz(test_value[i]));
        }
    }

    #[test]
    fn test_iterator_for_loop() {
        let aux = Collatz::new(133);
        let sequence = vec![
            133, 400, 200, 100, 50, 25, 76, 38, 19, 58, 29, 88, 44, 22, 11, 34, 17, 52, 26, 13, 40,
            20, 10, 5, 16, 8, 4, 2, 1,
        ];

        for (i, value) in aux.enumerate() {
            assert_eq!(value.v, sequence[i]);
        }
    }

    #[test]
    fn test_iterator_count() {
        let nb = Collatz::new(133);
        assert_eq!(nb.count(), 28);
    }
}
