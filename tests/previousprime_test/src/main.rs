use previousprime::*;

fn main() {
    println!("The previous prime number before 34 is: {}", prev_prime(34));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prev_prime_test() {
        assert_eq!(0, prev_prime(0));
        assert_eq!(0, prev_prime(2));
        assert_eq!(2, prev_prime(3));
        assert_eq!(3, prev_prime(5));
        assert_eq!(31, prev_prime(34));
        assert_eq!(631, prev_prime(633));
        assert_eq!(478139, prev_prime(478152));
    }
}
